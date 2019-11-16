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
fn EVAL(mut ast: MalExpression, env: Rc<Env>) -> MalRet {
    //    let ast_str = pr_str(&ast, true);
    //    println!("EVAL: {}", ast_str);
    let mut loop_env = env.clone();
    //    let mut loop_count = 0;
    'tco: loop {
        //        if (loop_count > 0) {
        //            println!("EVAL_loop {}: {}", loop_count, pr_str(&ast, true));
        //        }
        //        loop_count = loop_count + 1;
        match ast.clone() {
            List(forms) => {
                if forms.is_empty() {
                    return Ok(ast);
                }
                let form0 = forms[0].clone();
                let rest_forms: Vec<MalExpression> = forms[1..].to_vec().clone();
                match form0 {
                    Symbol(ref sym) if sym == "def!" => {
                        return handle_def(rest_forms.to_vec(), loop_env)
                    }
                    Symbol(ref sym) if sym == "let*" => {
                        match (rest_forms.get(0), rest_forms.get(1)) {
                            (Some(List(f0)), Some(f1)) | (Some(Vector(f0)), Some(f1)) => {
                                loop_env = Rc::new(Env::new(
                                    Some(env.clone()),
                                    Rc::new(vec![]),
                                    Rc::new(vec![]),
                                )?);
                                for chunk in f0.chunks(2) {
                                    if let Symbol(key) = &chunk[0] {
                                        let val = EVAL(chunk[1].clone(), loop_env.clone())?;
                                        loop_env.set(key, val);
                                    } else {
                                        return Err(format!(
                                            "let* sub-argument not a symbol: {}",
                                            pr_str(&chunk[0], true)
                                        ));
                                    }
                                }
                                ast = f1.clone();
                                continue 'tco;
                            }
                            _ => {
                                return Err(
                                    "let* needs 2 arguments; first should be a list or vector"
                                        .to_string(),
                                )
                            }
                        }
                    }
                    Symbol(ref sym) if sym == "do" => {
                        if !rest_forms.is_empty() {
                            for x in rest_forms[0..(rest_forms.len() - 1)].iter() {
                                let evaled = EVAL(x.clone(), loop_env.clone());
                                if evaled.is_err() {
                                    return evaled;
                                }
                            }

                            ast = rest_forms[rest_forms.len() - 1].clone();
                        // env unchanged
                        } else {
                            return Ok(Nil());
                        }
                    }
                    Symbol(ref sym) if sym == "if" => {
                        return match (rest_forms.get(0), rest_forms.get(1)) {
                            (Some(condition), Some(eval_if_true)) => {
                                if EVAL(condition.clone(), loop_env.clone())?.is_true_in_if() {
                                    ast = eval_if_true.clone();
                                    continue 'tco;
                                } else {
                                    match rest_forms.get(2) {
                                        Some(eval_if_false) => {
                                            ast = eval_if_false.clone();
                                            continue 'tco;
                                        }
                                        None => Ok(Nil()),
                                    }
                                }
                            }
                            _ => Err("if expression must have at least two arguments".to_string()),
                        }
                    }
                    Symbol(ref sym) if sym == "fn*" => {
                        return handle_fn(rest_forms.to_vec(), loop_env)
                    }
                    RustFunction(f) => {
                        if let List(rest_evaled) =
                            eval_ast(&List(Rc::new(rest_forms.to_vec())), loop_env)?
                        {
                            //                            println!(
                            //                                "applying f({})",
                            //                                rest_evaled
                            //                                    .to_vec()
                            //                                    .iter()
                            //                                    .map(|x| pr_str(x, true))
                            //                                    .join(", ")
                            //                            );
                            return f(rest_evaled.to_vec());
                        } else {
                            panic!("eval_ast List -> non-List")
                        }
                    }
                    FnFunction {
                        binds,
                        ast: fn_ast,
                        outer_env,
                        ..
                    } => {
                        let f_args = eval_ast(&List(Rc::new(rest_forms.to_vec())), loop_env)?;
                        match f_args {
                            List(f_args_vec) => {
                                let binds_vec_string: Vec<String> = binds
                                    .iter()
                                    .map(|x| match x {
                                        MalExpression::Symbol(x_symbol) => x_symbol.clone(),
                                        _ => panic!(
                                            "non-symbol {} in FnFunction binds",
                                            pr_str(x, true)
                                        ),
                                    })
                                    .collect();
                                let fn_env = Env::new(
                                    Some(outer_env.clone()),
                                    Rc::new(binds_vec_string),
                                    f_args_vec,
                                )?;
                                ast = (*fn_ast).clone();
                                loop_env = Rc::new(fn_env);
                                continue 'tco;
                            }
                            _ => panic!("eval_ast(List) => non-List"),
                        }
                    }
                    Symbol(_) | List(_) => match EVAL(form0, loop_env.clone()) {
                        Ok(List(ref x)) if x.is_empty() => {
                            return Err("Cannot apply empty list as function".to_string())
                        }
                        Ok(form0_evaled) => {
                            let mut spliced_ast = vec![form0_evaled];
                            spliced_ast.append(&mut (&forms[1..]).to_vec());
                            ast = List(Rc::new(spliced_ast));
                            continue 'tco;
                        }
                        Err(e) => return Err(e),
                    },
                    other => {
                        return Err(format!(
                            "not a symbol, list, or function: {}",
                            pr_str(&other, true)
                        ))
                    }
                }
            }
            _ => return eval_ast(&ast, loop_env),
        }
    }
}

fn handle_def(forms: Vec<MalExpression>, env: Rc<Env>) -> MalRet {
    match (forms.get(0), forms.get(1)) {
        (Some(Symbol(f0)), Some(f1)) => {
            let key = f0;
            let value = EVAL(f1.clone(), env.clone())?;
            env.set(key, value.clone());
            Ok(value)
        }
        _ => Err("def! requires 2 arguments; first argument should be a symbol".to_string()),
    }
}

fn handle_fn(forms: Vec<MalExpression>, env: Rc<Env>) -> MalRet {
    match (forms.get(0), forms.get(1)) {
        (Some(List(f0_v)), Some(f1)) | (Some(Vector(f0_v)), Some(f1)) => Ok(FnFunction {
            binds: f0_v.clone(),
            ast: Rc::new(f1.clone()),
            outer_env: env,
            is_macro: false
        }),
        _ => Err(
            "fn* expression must have at least two arguments; first must be list or vector"
                .to_string(),
        ),
    }
}

fn eval_ast(ast: &MalExpression, env: Rc<Env>) -> MalRet {
    //    let ast_str = pr_str(ast, true);
    //    println!("eval_ast: {}", ast_str);
    match ast.clone() {
        Symbol(ref symbol) => {
            let get = env.get(&symbol);
            match get {
                Some(result) => Ok(result),
                None => Err(format!("symbol {} not found in environment", symbol)),
            }
        }
        List(list) => match iterate_rc_vec(list).map(|x| EVAL(x, env.clone())).collect() {
            Ok(collected) => Ok(List(Rc::new(collected))),
            Err(e) => Err(e),
        },
        Vector(vector) => match iterate_rc_vec(vector)
            .map(|x| EVAL(x, env.clone()))
            .collect()
        {
            Ok(collected) => Ok(Vector(Rc::new(collected))),
            Err(e) => Err(e),
        },
        HashTable(hash_table) => match iterate_rc_vec(hash_table)
            .map(|x| EVAL(x, env.clone()))
            .collect()
        {
            Ok(collected) => Ok(HashTable(Rc::new(collected))),
            Err(e) => Err(e),
        },
        _ => Ok(ast.clone()),
    }
}

#[allow(non_snake_case)]
fn PRINT(form: MalRet) -> Result<String, String> {
    Ok(pr_str(&form?, true))
}

fn rep(line: &str, env: Rc<Env>) -> Result<String, String> {
    PRINT(EVAL(READ(line)?, env))
}

fn main() {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    let env = Rc::new(core::core_ns());

    if rl.load_history(".mal-history").is_err() {
        eprintln!("No previous history.");
    }

    // functions defined in MAL
    match rep("(def! not (fn* (a) (if a false true)))", env.clone()) {
        Ok(_) => {}
        Err(e) => panic!("Error in internal function setup: {}", e),
    }

    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                rl.save_history(".mal-history").unwrap();
                if !line.is_empty() {
                    match rep(&line.to_owned(), env.clone()) {
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
        let env = Rc::new(core::core_ns());
        assert_eq!(rep("(do (def! sum2 (fn* (n acc) (if (= n 0) acc (sum2 (- n 1) (+ n acc))))) (sum2 50 0))", env), Ok("1275".to_string()));
    }
}
