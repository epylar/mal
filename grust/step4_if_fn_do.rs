pub mod core;
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
use types::iterate_rc_vec;
use types::MalExpression;
use types::MalExpression::{FnFunction, HashTable, List, Nil, RustFunction, Symbol, Vector};
use types::MalRet;

#[allow(non_snake_case)]
fn READ(input: &str) -> MalRet {
    //    println!("READ: {}", input);
    read_str(input)
}

#[allow(non_snake_case)]
fn EVAL(ast: &MalExpression, env: &mut Env) -> MalRet {
    // println!("EVAL: {}", pr_str(&ast));
    match ast.clone() {
        List(forms) => {
            if forms.is_empty() {
                return Ok(ast.clone());
            }
            let form0 = &forms[0];
            let rest_forms = &forms[1..];
            match form0 {
                Symbol(sym) if sym == "def!" => handle_def(rest_forms.to_vec(), env),
                Symbol(sym) if sym == "let*" => handle_let(rest_forms.to_vec(), env),
                Symbol(sym) if sym == "do" => handle_do(rest_forms.to_vec(), env),
                Symbol(sym) if sym == "if" => handle_if(rest_forms.to_vec(), env),
                Symbol(sym) if sym == "fn*" => handle_fn(rest_forms.to_vec(), env.clone()),
                RustFunction(f) => {
                    if let List(rest_evaled) = eval_ast(&List(Rc::new(rest_forms.to_vec())), env)? {
                        f(rest_evaled.to_vec())
                    } else {
                        panic!("eval_ast List -> non-List")
                    }
                }
                FnFunction {
                    binds,
                    ast,
                    outer_env,
                } => {
                    let rest_evaled = eval_ast(&List(Rc::new((&forms[1..]).to_vec())), env)?;
                    match rest_evaled {
                        List(rest_evaled_vec) => {
                            let binds_vec_string: Vec<String> = binds
                                .iter()
                                .map(|x| match x {
                                    MalExpression::Symbol(x_symbol) => x_symbol.clone(),
                                    _ => panic!("non-symbol {} in FnFunction binds", pr_str(x)),
                                })
                                .collect();
                            let mut fn_env = Env::new(
                                Some(outer_env.clone()),
                                Rc::new(binds_vec_string),
                                rest_evaled_vec,
                            );
                            EVAL(&ast, &mut fn_env)
                        }
                        _ => panic!("eval_ast(List) => non-List"),
                    }
                }
                Symbol(_) | List(_) => match EVAL(form0, env) {
                    Ok(form0_evaled) => {
                        let mut spliced_ast = vec![form0_evaled];
                        spliced_ast.append(&mut (&forms[1..]).to_vec());
                        EVAL(&List(Rc::new(spliced_ast)), env)
                    }
                    Err(e) => Err(e),
                },
                other => Err(format!(
                    "not a symbol, list, or function: {}",
                    pr_str(other)
                )),
            }
        }
        _ => eval_ast(&ast, env),
    }
}

fn handle_let(forms: Vec<MalExpression>, env: &mut Env) -> MalRet {
    match (forms.get(0), forms.get(1)) {
        (Some(List(f0)), Some(f1)) | (Some(Vector(f0)), Some(f1)) => {
            let mut newenv = Env::new(Some(env.clone()), Rc::new(vec![]), Rc::new(vec![]));
            for chunk in f0.chunks(2) {
                if let Symbol(key) = &chunk[0] {
                    let val = EVAL(&chunk[1], &mut newenv)?;
                    newenv.set(key, val);
                } else {
                    return Err(format!(
                        "let* sub-argument not a symbol: {}",
                        pr_str(&chunk[0])
                    ));
                }
            }
            EVAL(f1, &mut newenv)
        }
        _ => Err("let* needs 2 arguments; first should be a list or vector".to_string()),
    }
}

fn handle_def(forms: Vec<MalExpression>, env: &mut Env) -> MalRet {
    match (forms.get(0), forms.get(1)) {
        (Some(Symbol(f0)), Some(f1)) => {
            let key = f0;
            let value = EVAL(f1, env)?;
            env.set(key, value.clone());
            Ok(value)
        }
        _ => Err("def! requires 2 arguments; first argument should be a symbol".to_string()),
    }
}

fn handle_do(forms: Vec<MalExpression>, env: &mut Env) -> MalRet {
    let mut evaled_x = Ok(Nil());
    for x in forms {
        evaled_x = EVAL(&x, env);
        if evaled_x.is_err() {
            break;
        }
    }
    evaled_x
}

fn handle_if(forms: Vec<MalExpression>, env: &mut Env) -> MalRet {
    match (forms.get(0), forms.get(1)) {
        (Some(condition), Some(eval_if_true)) => {
            if EVAL(condition, env)?.is_true_in_if() {
                EVAL(eval_if_true, env)
            } else {
                match forms.get(2) {
                    Some(eval_if_false) => EVAL(eval_if_false, env),
                    None => Ok(Nil()),
                }
            }
        }
        _ => Err("if expression must have at least two arguments".to_string()),
    }
}

fn handle_fn(forms: Vec<MalExpression>, env: Env) -> MalRet {
    match (forms.get(0), forms.get(1)) {
        (Some(List(f0_v)), Some(f1)) | (Some(Vector(f0_v)), Some(f1)) => Ok(FnFunction {
            binds: f0_v.clone(),
            ast: Rc::new(f1.clone()),
            outer_env: env,
        }),
        _ => Err(
            "fn* expression must have at least two arguments; first must be list or vector"
                .to_string(),
        ),
    }
}

fn eval_ast(ast: &MalExpression, env: &mut Env) -> MalRet {
    // println!("eval_ast: {}", pr_str(&ast));
    match ast.clone() {
        Symbol(symbol) => {
            let get = env.get(&symbol);
            match get {
                Some(result) => {
                    println!("symbol lookup: {}", pr_str(&result));
                    Ok(result.clone())
                }
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
        HashTable(hash_table) => {
            match iterate_rc_vec(hash_table).map(|x| EVAL(&x, env)).collect() {
                Ok(collected) => Ok(HashTable(Rc::new(collected))),
                Err(e) => Err(e),
            }
        }
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

fn main() {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    let mut env = core::core_ns();

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
    fn test_do() {
        let mut env = core::core_ns();
        assert_eq!(rep("(do 1 2 3)", &mut env), Ok("3".to_string()));
        assert_eq!(rep("(do 2)", &mut env), Ok("2".to_string()));
        assert_eq!(rep("(do)", &mut env), Ok("nil".to_string()));
        assert_eq!(rep("(do 4 (+ 1 2))", &mut env), Ok("3".to_string()));
    }
}
