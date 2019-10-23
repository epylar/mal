pub mod env;
pub mod printer;
pub mod reader;
pub mod types;

use crate::env::Env;
use printer::pr_str;
use reader::read_str;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;
use std::rc::Rc;
use types::MalExpression;
use types::MalExpression::{Function, HashTable, Int, List, Symbol, Vector};
use types::MalRet;

#[allow(non_snake_case)]
fn READ(input: &str) -> MalRet {
    //    println!("READ: {}", input);
    read_str(input)
}

#[allow(non_snake_case)]
fn EVAL(ast: &MalExpression, env: &mut Env) -> MalRet {
    //    println!("EVAL: {}", pr_str(&ast));
    match ast.clone() {
        List(l) => {
            if l.is_empty() {
                return Ok(ast.clone());
            }
            let l0 = &l[0];
            match l0 {
                Symbol(sym) if sym == "def!" => {
                    if l.len() != 3 { return Err("def! requires exactly 2 arguments".to_string()) }
                    let key = &l[1];
                    let value = EVAL(&l[2], env)?;
                    match key {
                        Symbol(key_symbol) => {
                            env.set(&key_symbol, value.clone());
                            Ok(value.clone())
                        }
                        _ => Err(format!(
                            "attempting to def! with a non-symbol: {}",
                            pr_str(&key)
                        )),
                    }
                }
                Symbol(_) => match EVAL(l0, env) {
                    Ok(Function(f)) => {
                        let rest_evaled = eval_ast(&List(Rc::new((&l[1..]).to_vec())), env)?;
                        f(rest_evaled)
                    }
                    Err(e) => Err(e),
                    Ok(other) => Err(format!("Not a function: {}", pr_str(&other))),
                },
                other => Err(format!("not a symbol: {}", pr_str(other))),
            }
        }
        _ => eval_ast(&ast, env),
    }
}

fn eval_ast(ast: &MalExpression, env: &mut Env) -> MalRet {
    //    println!("eval_ast: {}", pr_str(&ast));
    match ast.clone() {
        Symbol(symbol) => {
            let get = env.get(&symbol);
            match get {
                Some(result) => Ok(result.clone()),
                None => Err(format!("symbol {} not found in environment", symbol)),
            }
        }
        List(list) => match iterate_rc_vec(list).map(|x| EVAL(&x, env)).collect() {
            Ok(collected) => Ok(List(Rc::new(collected))),
            Err(e) => Err(e),
        },
        Vector(vector) => match iterate_rc_vec(vector).map(|x| EVAL(&x, env)).collect() {
            Ok(collected) => Ok(Vector(Rc::new(collected))),
            Err(e) => Err(e),
        },
        HashTable(hash_table) => match iterate_rc_vec(hash_table).map(|x| EVAL(&x, env)).collect() {
            Ok(collected) => Ok(HashTable(Rc::new(collected))),
            Err(e) => Err(e),
        },
        _ => Ok(ast.clone()),
    }
}

#[allow(non_snake_case)]
fn PRINT(form: MalRet) -> Result<String, String> {
    Ok(pr_str(&form?))
}

fn rep(line: &str, env: &mut Env) -> Result<String, String> {
    PRINT(EVAL(&READ(line)?, env))
}

fn plus(args: MalExpression) -> MalRet {
    mal_int_fn(args, |a, b| a + b, 0)
}

fn minus(args: MalExpression) -> MalRet {
    mal_int_fn_binary(args, |a, b| a - b)
}

fn times(args: MalExpression) -> MalRet {
    mal_int_fn(args, |a, b| a * b, 1)
}

fn int_divide(args: MalExpression) -> MalRet {
    mal_int_fn_binary(args, |a, b| a / b)
}

fn mal_int_fn_binary(args: MalExpression, func: fn(i32, i32) -> i32) -> MalRet {
    if let List(l) = args {
        match (&l[0], &l[1]) {
            (Int(a), Int(b)) => Ok(Int(func(*a, *b))),
            _ => Err("invalid arguments to binary int function".to_string()),
        }
    } else {
        Err("function called with non-list".to_string())
    }
}

fn iterate_rc_vec(data: Rc<Vec<MalExpression>>) -> impl Iterator<Item = MalExpression> {
    let len = data.len();
    (0..len).map(move |i| data[i].clone())
}

fn mal_int_fn(args: MalExpression, func: fn(i32, i32) -> i32, initial: i32) -> MalRet {
    if let List(l) = args {
        let mut result = initial;
        for x in iterate_rc_vec(l) {
            match x {
                Int(x_int) => result = func(result, x_int),
                _ => return Err("function called with non-int".to_string()),
            }
        }
        Ok(Int(result))
    } else {
        Err("function called with non-list".to_string())
    }
}

fn init_env() -> Env {
    let mut env = Env::new(None, HashMap::new());

    env.set("+", Function(plus));
    env.set("-", Function(minus));
    env.set("*", Function(times));
    env.set("/", Function(int_divide));

    env
}

fn main() {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    let mut env = init_env();

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
                    match rep(&line.to_owned(), &mut env) {
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
        let mut env = init_env();
        //        assert_eq!(rep("1", &env), Ok("1".to_string()));
        assert_eq!(rep("(+ 1)", &mut env), Ok("1".to_string()));
        //        assert_eq!(rep("(+ 1 2)", &env), Ok("3".to_string()));
        //        assert_eq!(rep("(+ 1 2 3)", &env), Ok("6".to_string()));
        //        assert_eq!(rep("\":a\"", &env), Ok("\":a\"".to_string()));
    }
}
