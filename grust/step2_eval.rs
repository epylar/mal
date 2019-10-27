extern crate itertools;
extern crate lazy_static;
extern crate regex;
extern crate rustyline;

pub mod env;
pub mod printer;
pub mod reader;
pub mod types;

use printer::pr_str;
use reader::read_str;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;
use std::rc::Rc;
use types::MalExpression;
use types::MalExpression::{HashTable, Int, List, RustFunction, Symbol, Vector};
use types::MalRet;

type Env = HashMap<String, MalExpression>;

#[allow(non_snake_case)]
fn READ(input: &str) -> MalRet {
    read_str(input)
}

#[allow(non_snake_case)]
fn EVAL(ast: MalExpression, env: &Env) -> MalRet {
    //    println!("EVAL: {}", pr_str(&ast));
    match ast.clone() {
        List(l) => {
            if l.is_empty() {
                return Ok(ast);
            }
            let l0 = &l[0];
            match l0 {
                Symbol(_) => match EVAL(l0.clone(), env) {
                    Ok(RustFunction(f)) => {
                        if let List(rest_evaled) = eval_ast(List(Rc::new((&l[1..]).to_vec())), env)?
                        {
                            f(rest_evaled.to_vec())
                        } else {
                            panic!("eval_ast List -> non-List")
                        }
                    }
                    Err(e) => Err(e),
                    Ok(other) => Err(format!("Not a function: {}", pr_str(&other))),
                },
                other => Err(format!("not a symbol: {}", pr_str(other))),
            }
        }
        _ => eval_ast(ast, env),
    }
}

fn eval_ast(ast: MalExpression, env: &Env) -> MalRet {
    //    println!("eval_ast: {}", pr_str(&ast));
    match ast {
        Symbol(symbol) => {
            let get = env.get(&symbol);
            match get {
                Some(result) => Ok(result.clone()),
                None => Err(format!("symbol {} not found in environment", symbol)),
            }
        }
        List(list) => match iterate_rc_vec(list).map(|x| EVAL(x, env)).collect() {
            Ok(collected) => Ok(List(Rc::new(collected))),
            Err(e) => Err(e),
        },
        Vector(vector) => match iterate_rc_vec(vector).map(|x| EVAL(x, env)).collect() {
            Ok(collected) => Ok(Vector(Rc::new(collected))),
            Err(e) => Err(e),
        },
        HashTable(hash_table) => match iterate_rc_vec(hash_table).map(|x| EVAL(x, env)).collect() {
            Ok(collected) => Ok(HashTable(Rc::new(collected))),
            Err(e) => Err(e),
        },
        _ => Ok(ast),
    }
}

#[allow(non_snake_case)]
fn PRINT(form: MalRet) -> Result<String, String> {
    Ok(pr_str(&form?))
}

fn rep(line: &str, env: &Env) -> Result<String, String> {
    PRINT(EVAL(READ(line)?, env))
}

fn plus(args: Vec<MalExpression>) -> MalRet {
    mal_int_fn(args, |a, b| a + b, 0)
}

fn minus(args: Vec<MalExpression>) -> MalRet {
    mal_int_fn_binary(args, |a, b| a - b)
}

fn times(args: Vec<MalExpression>) -> MalRet {
    mal_int_fn(args, |a, b| a * b, 1)
}

fn int_divide(args: Vec<MalExpression>) -> MalRet {
    mal_int_fn_binary(args, |a, b| a / b)
}

fn mal_int_fn_binary(args: Vec<MalExpression>, func: fn(i32, i32) -> i32) -> MalRet {
    match (args.get(0), args.get(1)) {
        (Some(Int(a)), Some(Int(b))) => Ok(Int(func(*a, *b))),
        _ => Err("invalid arguments to binary int function".to_string()),
    }
}

fn mal_int_fn(args: Vec<MalExpression>, func: fn(i32, i32) -> i32, initial: i32) -> MalRet {
    let mut result = initial;
    for x in args {
        match x {
            Int(x_int) => result = func(result, x_int),
            _ => return Err("function called with non-int".to_string()),
        }
    }
    Ok(Int(result))
}

fn iterate_rc_vec(data: Rc<Vec<MalExpression>>) -> impl Iterator<Item = MalExpression> {
    let len = data.len();
    (0..len).map(move |i| data[i].clone())
}

fn init_env() -> Env {
    let mut env: Env = HashMap::new();

    env.insert("+".to_string(), RustFunction(plus));
    env.insert("-".to_string(), RustFunction(minus));
    env.insert("*".to_string(), RustFunction(times));
    env.insert("/".to_string(), RustFunction(int_divide));

    env
}

fn main() {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    let env = init_env();

    if rl.load_history(".mal-history").is_err() {
        eprintln!("No previous history.");
    }

    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                rl.save_history(".mal-history").unwrap();
                if !line.is_empty() {
                    match rep(&line.to_owned(), &env) {
                        Ok(result) => println!("{}", result),
                        Err(e) => println!("error: {}", e),
                    }
                }
            }
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_rep() {
        let env = init_env();
        assert_eq!(rep("1", &env), Ok("1".to_string()));
        assert_eq!(rep("(+ 1)", &env), Ok("1".to_string()));
        assert_eq!(rep("(+ 1 2)", &env), Ok("3".to_string()));
        assert_eq!(rep("(+ 1 2 3)", &env), Ok("6".to_string()));
        assert_eq!(rep("\":a\"", &env), Ok("\":a\"".to_string()));
    }
}
