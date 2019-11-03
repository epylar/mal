pub mod core;
pub mod env;
pub mod printer;
pub mod reader;
pub mod types;

#[macro_use]
extern crate log;

use crate::env::Env;
use crate::types::MalExpression::Atom;
use log::debug;
use log::Level;
use printer::pr_str;
use reader::read_str;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::rc::Rc;
use types::iterate_rc_vec;
use types::Closure;
use types::MalExpression;
use types::MalExpression::{
    FnFunction, HashTable, List, Nil, RustClosure, RustFunction, Symbol, Tco, Vector,
};
use types::MalRet;

#[allow(non_snake_case)]
fn READ(input: &str) -> MalRet {
    debug!("READ: {}", input);
    read_str(input)
}

#[allow(non_snake_case)]
fn EVAL(mut ast: MalExpression, env: Rc<Env>) -> MalRet {
    if log_enabled!(Level::Debug) {
        let ast_str = pr_str(&ast, true);
        debug!("EVAL: {}", ast_str);
    }
    let mut loop_env = env;
    let mut loop_count = 0;
    'tco: loop {
        if loop_count > 0 && log_enabled!(Level::Debug) {
            debug!("EVAL_loop {}: {}", loop_count, pr_str(&ast, true));
        }
        loop_count += 1;
        match ast.clone() {
            Tco(exp, env) => {
                loop_env = env;
                ast = *exp;
                continue 'tco;
            }
            List(forms) => {
                if forms.is_empty() {
                    return Ok(ast);
                }
                let form0 = forms[0].clone();
                let rest_forms: Vec<MalExpression> = forms[1..].to_vec().clone();
                let loop_result: MalRet = match form0 {
                    Symbol(ref sym) if sym == "def!" => handle_def(rest_forms.to_vec(), loop_env),
                    Symbol(ref sym) if sym == "let*" => handle_let(rest_forms.to_vec(), loop_env),
                    Symbol(ref sym) if sym == "do" => handle_do(rest_forms.to_vec(), loop_env),
                    Symbol(ref sym) if sym == "if" => handle_if(rest_forms.to_vec(), loop_env),
                    Symbol(ref sym) if sym == "fn*" => handle_fn(rest_forms.to_vec(), loop_env),
                    Symbol(ref sym) if sym == "swap!" => handle_swap(rest_forms.to_vec(), loop_env),
                    Symbol(ref sym) if sym == "quote" => {
                        handle_quote(rest_forms.to_vec(), loop_env)
                    }
                    Symbol(ref sym) if sym == "quasiquote" => {
                        handle_quasiquote(rest_forms.to_vec(), loop_env)
                    }
                    RustFunction(f) => {
                        if let List(rest_evaled) =
                            eval_ast(&List(Rc::new(rest_forms.to_vec())), loop_env)?
                        {
                            return f(rest_evaled.to_vec());
                        } else {
                            panic!("eval_ast List -> non-List")
                        }
                    }
                    RustClosure(c) => match rest_forms.get(0) {
                        Some(arg) => {
                            let abc = c.0;
                            return (abc)(
                                EVAL(arg.clone(), loop_env.clone())?,
                                Env::get_env_top_level(loop_env),
                            );
                        }
                        None => return Err("argument required".to_string()),
                    },
                    FnFunction {
                        binds,
                        ast: fn_ast,
                        outer_env,
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
                        Ok(List(x)) if x.is_empty() => {
                            Err("Cannot apply empty list as function".to_string())
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
                };
                match loop_result {
                    Ok(Tco(exp, env)) => {
                        loop_env = env;
                        ast = *exp;
                        continue 'tco;
                    }
                    x => return x,
                }
            }
            _ => return eval_ast(&ast, loop_env),
        }
    }
}

fn handle_let(forms: Vec<MalExpression>, env: Rc<Env>) -> MalRet {
    match (forms.get(0), forms.get(1)) {
        (Some(List(f0)), Some(f1)) | (Some(Vector(f0)), Some(f1)) => {
            let let_env = Rc::new(Env::new(Some(env), Rc::new(vec![]), Rc::new(vec![]))?);
            for chunk in f0.chunks(2) {
                if let Symbol(key) = &chunk[0] {
                    let val = EVAL(chunk[1].clone(), let_env.clone())?;
                    let_env.set(key, val);
                } else {
                    return Err(format!(
                        "let* sub-argument not a symbol: {}",
                        pr_str(&chunk[0], true)
                    ));
                }
            }
            Ok(Tco(Box::new(f1.clone()), let_env))
        }
        _ => Err("let* needs 2 arguments; first should be a list or vector".to_string()),
    }
}

fn handle_do(forms: Vec<MalExpression>, env: Rc<Env>) -> MalRet {
    if !forms.is_empty() {
        for x in forms[0..(forms.len() - 1)].iter() {
            let evaled = EVAL(x.clone(), env.clone());
            if evaled.is_err() {
                return evaled;
            }
        }

        Ok(Tco(Box::new(forms[forms.len() - 1].clone()), env))
    } else {
        Ok(Nil())
    }
}

fn handle_if(forms: Vec<MalExpression>, env: Rc<Env>) -> MalRet {
    match (forms.get(0), forms.get(1)) {
        (Some(condition), Some(eval_if_true)) => {
            if EVAL(condition.clone(), env.clone())?.is_true_in_if() {
                Ok(Tco(Box::new(eval_if_true.clone()), env))
            } else {
                match forms.get(2) {
                    Some(eval_if_false) => Ok(Tco(Box::new(eval_if_false.clone()), env)),
                    None => Ok(Nil()),
                }
            }
        }
        _ => Err("if expression must have at least two arguments".to_string()),
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

fn handle_swap(forms: Vec<MalExpression>, env: Rc<Env>) -> MalRet {
    let forms_evaled = eval_ast(&List(Rc::new(forms.clone())), env.clone())?;
    let forms_evaled_vec: Vec<MalExpression> = iterate_rc_vec(match forms_evaled {
        List(l) => l,
        _ => panic!("EVAL List -> non-list"),
    })
    .collect();
    match (forms_evaled_vec.get(0), forms_evaled_vec.get(1)) {
        (Some(Atom(a)), Some(b)) => {
            let atom_val = a.borrow().clone();
            let mut args = vec![b.clone(), atom_val];

            args.append(&mut (&forms_evaled_vec[2..]).to_vec());
            let form_to_eval = List(Rc::new(args));
            let replacement = EVAL(form_to_eval, env);
            a.replace(replacement.clone()?);
            replacement
        }
        _ => Err(format!("swap! -- bad arguments: {:?}", forms)),
    }
}

fn handle_fn(forms: Vec<MalExpression>, env: Rc<Env>) -> MalRet {
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

fn handle_quote(forms: Vec<MalExpression>, _env: Rc<Env>) -> MalRet {
    match forms.get(0) {
        Some(x) => Ok(x.clone()),
        None => Err("quote requires an argument".to_string()),
    }
}

fn handle_quasiquote(forms: Vec<MalExpression>, env: Rc<Env>) -> MalRet {
    let result = handle_quasiquote_inner(forms, env.clone());
    Ok(Tco(Box::new(result?), env))
}

fn handle_quasiquote_inner(forms: Vec<MalExpression>, env: Rc<Env>) -> MalRet {
    if let Some(x) = forms.get(0) {
        debug!("handle_quasiquote_inner: {}", pr_str(x, true));
    } else {
        debug!("handle_quasiquote_inner: <nothing>");
    }
    let result = match forms.get(0) {
        Some(List(list_contents)) if !list_contents.is_empty() => match &list_contents[0] {
            Symbol(s) if s == "unquote" => match list_contents.get(1) {
                Some(x) => Ok(Tco(Box::new(x.clone()), env)),
                None => Err("unquote requires an argument".to_string()),
            },
            List(l) => match (l.get(0), l.get(1)) {
                (Some(Symbol(s)), Some(arg)) if s == "splice-unquote" => {
                    let concat = Symbol("concat".to_string());
                    let concat_1 = arg.clone();
                    let concat_2 = handle_quasiquote_inner(
                        vec![List(Rc::new(list_contents[1..].to_vec()))],
                        env,
                    )?;
                    Ok(List(Rc::new(vec![concat, concat_1, concat_2])))
                }
                (Some(Symbol(s)), None) if s == "splice-unquote" => {
                    Err("splice-unquote requires an argument".to_string())
                }
                _ => handle_quasiquote_inner_cons_case(list_contents, env),
            },
            _ => handle_quasiquote_inner_cons_case(list_contents, env),
        },
        None => Ok(List(Rc::new(vec![]))),
        Some(x) => Ok(List(Rc::new(vec![Symbol("quote".to_string()), x.clone()]))),
    };
    debug!(
        "handle_quasiquote_inner result: {}",
        pr_str(&(result.clone()?), true)
    );
    result
}

fn handle_quasiquote_inner_cons_case(
    list_contents: &Rc<Vec<MalExpression>>,
    env: Rc<Env>,
) -> MalRet {
    let quasi_first = handle_quasiquote_inner(vec![list_contents[0].clone()], env.clone())?;
    let quasi_rest =
        handle_quasiquote_inner(vec![List(Rc::new(list_contents[1..].to_vec()))], env)?;
    Ok(List(Rc::new(vec![
        Symbol("cons".to_string()),
        quasi_first,
        quasi_rest,
    ])))
}

fn eval_ast(ast: &MalExpression, env: Rc<Env>) -> MalRet {
    if log_enabled!(Level::Debug) {
        let ast_str = pr_str(ast, true);
        debug!("eval_ast: {}", ast_str);
    }
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
    env_logger::init();

    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    let env = Rc::new(core::core_ns());
    let rust_eval_closure = RustClosure(Closure(Rc::new(EVAL)));
    env.set("eval", rust_eval_closure);

    if rl.load_history(".mal-history").is_err() {
        eprintln!("No previous history.");
    }

    // functions defined in MAL
    match rep(
        r#"(do (def! not (fn* (a) (if a false true))) (def! load-file (fn* (f) (eval (read-string (str "(do " (slurp f) "\nnil)"))))))"#,
        env.clone(),
    ) {
        Ok(_) => {}
        Err(e) => panic!("Error in internal function setup: {}", e),
    }

    let args: Vec<String> = std::env::args().collect();
    env.set(
        "*ARGV*",
        if args.len() > 2 {
            let argv_vec: Vec<MalExpression> = (&args[2..])
                .iter()
                .map(|x| MalExpression::String(x.to_string()))
                .collect();
            List(Rc::new(argv_vec))
        } else {
            List(Rc::new(vec![]))
        },
    );

    if let Some(filename) = args.get(1) {
        let quoted_filename: &str = &crate::printer::pr_str_slice(filename, true);
        let mal_load_file: &str = &format!("(load-file {})", quoted_filename);
        if let Err(e) = rep(mal_load_file, env) {
            panic!("Error loading file {}: {}", filename, e)
        }
    } else {
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
