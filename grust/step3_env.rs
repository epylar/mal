pub mod env;
pub mod printer;
pub mod reader;
pub mod types;

use crate::env::Env;
use printer::pr_str;
use reader::read_str;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::rc::Rc;
use types::MalExpression;
use types::MalExpression::{HashTable, Int, List, RustFunction, Symbol, Vector};
use types::MalRet;

#[allow(non_snake_case)]
fn READ(input: &str) -> MalRet {
    //    println!("READ: {}", input);
    read_str(input)
}

#[allow(non_snake_case)]
fn EVAL(ast: &MalExpression, env: Rc<Env>) -> MalRet {
    //        println!("EVAL: {}", pr_str(&ast));
    match ast.clone() {
        List(l) => {
            if l.is_empty() {
                return Ok(ast.clone());
            }
            let l0 = &l[0];
            match l0 {
                Symbol(ref sym) if sym == "def!" => {
                    if l.len() != 3 {
                        return Err("def! requires exactly 2 arguments".to_string());
                    }
                    let key = &l[1];
                    let value = EVAL(&l[2], env.clone())?;
                    match key {
                        Symbol(key_symbol) => {
                            env.set(&key_symbol, value.clone());
                            Ok(value)
                        }
                        _ => Err(format!(
                            "attempting to def! with a non-symbol: {}",
                            pr_str(&key, true)
                        )),
                    }
                }
                Symbol(ref sym) if sym == "let*" => match (l.get(1), l.get(2)) {
                    (Some(List(l1)), Some(l2)) | (Some(Vector(l1)), Some(l2)) => {
                        let newenv = Rc::new(Env::simple_new(Some(env))?);
                        for chunk in l1.chunks(2) {
                            if let Symbol(l_sym) = &chunk[0] {
                                let l_evaled_val = EVAL(&chunk[1], newenv.clone())?;
                                newenv.set(l_sym, l_evaled_val);
                            } else {
                                return Err(format!(
                                    "let* sub-argument not a symbol: {}",
                                    pr_str(&chunk[0], true)
                                ));
                            }
                        }
                        EVAL(l2, newenv)
                    }
                    _ => {
                        Err("let* needs 2 arguments; first should be a list or vector".to_string())
                    }
                },
                Symbol(_) => match EVAL(l0, env.clone()) {
                    Ok(RustFunction(f)) => {
                        if let List(rest_evaled) =
                            eval_ast(&List(Rc::new((&l[1..]).to_vec())), env)?
                        {
                            f(rest_evaled.to_vec())
                        } else {
                            panic!("eval_ast List -> non-List")
                        }
                    }
                    Err(e) => Err(e),
                    Ok(other) => Err(format!("Not a function: {}", pr_str(&other, true))),
                },
                other => Err(format!("not a symbol: {}", pr_str(other, true))),
            }
        }
        _ => eval_ast(&ast, env),
    }
}

fn eval_ast(ast: &MalExpression, env: Rc<Env>) -> MalRet {
    //    println!("eval_ast: {}", pr_str(&ast));
    match ast.clone() {
        Symbol(ref symbol) => {
            let get = env.get(&symbol);
            match get {
                Some(result) => Ok(result),
                None => Err(format!("symbol {} not found in environment", symbol)),
            }
        }
        List(list) => match iterate_rc_vec(list)
            .map(|x| EVAL(&x, env.clone()))
            .collect()
        {
            Ok(collected) => Ok(List(Rc::new(collected))),
            Err(e) => Err(e),
        },
        Vector(vector) => match iterate_rc_vec(vector)
            .map(|x| EVAL(&x, env.clone()))
            .collect()
        {
            Ok(collected) => Ok(Vector(Rc::new(collected))),
            Err(e) => Err(e),
        },
        HashTable(hash_table) => {
            match iterate_rc_vec(hash_table)
                .map(|x| EVAL(&x, env.clone()))
                .collect()
            {
                Ok(collected) => Ok(HashTable(Rc::new(collected))),
                Err(e) => Err(e),
            }
        }
        _ => Ok(ast.clone()),
    }
}

#[allow(non_snake_case)]
fn PRINT(form: MalRet) -> Result<String, String> {
    Ok(pr_str(&form?, true))
}

fn rep(line: &str, env: &mut Env) -> Result<String, String> {
    PRINT(EVAL(&READ(line)?, Rc::new(env.clone())))
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

fn init_env() -> Result<Env, String> {
    let env = Env::simple_new(None)?;

    env.set("+", RustFunction(plus));
    env.set("-", RustFunction(minus));
    env.set("*", RustFunction(times));
    env.set("/", RustFunction(int_divide));

    Ok(env)
}

fn main() {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    let mut env = match init_env() {
        Ok(e) => e,
        Err(e) => panic!("Error initializing environment: {}", e),
    };

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
        let mut env = init_env().unwrap();
        assert_eq!(rep("1", &mut env), Ok("1".to_string()));
        assert_eq!(rep("(+ 1)", &mut env), Ok("1".to_string()));
        assert_eq!(rep("(+ 1 2)", &mut env), Ok("3".to_string()));
        assert_eq!(rep("(+ 1 2 3)", &mut env), Ok("6".to_string()));
        assert_eq!(rep("\":a\"", &mut env), Ok("\":a\"".to_string()));
    }

    #[test]
    fn test_let() {
        let mut env = init_env().unwrap();
        assert_eq!(
            rep("(let* (p (+ 2 3) q (+ 2 p)) (+ p q))", &mut env),
            Ok("12".to_string())
        );
    }
}
