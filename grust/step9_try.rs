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
use tracing::instrument;
use types::iterate_rc_vec;
use types::list_from_vec;
use types::Closure;
use types::MalExpression;
use types::MalExpression::{
    Boolean, FnFunction, HashTable, Int, List, Nil, RustClosure, RustFunction, Symbol, Tco, Vector,
};
use types::MalRet;

#[allow(non_snake_case)]
fn READ(input: &str) -> MalRet {
    debug!("READ: {}", input);
    read_str(input)
}

#[allow(non_snake_case)]
#[instrument]
fn EVAL(mut ast: MalExpression, env: Rc<Env>) -> MalRet {
    let original_ast = ast.clone();
    if log_enabled!(Level::Debug) {
        debug!(">> EVAL: {}", pr_str(&ast, true));
    }
    let mut loop_env = env;
    let mut loop_count = 0;
    'tco: loop {
        if loop_count > 0 && log_enabled!(Level::Debug) {
            debug!("EVAL_loop {}: {}", loop_count, pr_str(&ast, true));
        }
        loop_count += 1;

        ast = macroexpand(&ast, loop_env.clone())?;

        match ast.clone() {
            Tco(exp, env) => {
                loop_env = env;
                ast = *exp;
                continue 'tco;
            }
            List(forms) => {
                if forms.is_empty() {
                    debug!(
                        "<< EVAL {} returning: {}",
                        pr_str(&original_ast, true),
                        pr_str(&ast.clone(), true)
                    );
                    return Ok(ast);
                }
                let form0 = forms[0].clone();
                let rest_forms: Vec<MalExpression> = forms[1..].to_vec().clone();
                let loop_result: MalRet = match form0 {
                    Symbol(ref sym) if sym == "def!" => eval_def(rest_forms.to_vec(), loop_env),
                    Symbol(ref sym) if sym == "defmacro!" => {
                        eval_defmacro(rest_forms.to_vec(), loop_env)
                    }
                    Symbol(ref sym) if sym == "let*" => eval_let(rest_forms.to_vec(), loop_env),
                    Symbol(ref sym) if sym == "do" => eval_do(rest_forms.to_vec(), loop_env),
                    Symbol(ref sym) if sym == "if" => eval_if(rest_forms.to_vec(), loop_env),
                    Symbol(ref sym) if sym == "fn*" => eval_fn(rest_forms.to_vec(), loop_env),
                    Symbol(ref sym) if sym == "swap!" => eval_swap(rest_forms.to_vec(), loop_env),
                    Symbol(ref sym) if sym == "quote" => eval_quote(rest_forms.to_vec(), loop_env),
                    Symbol(ref sym) if sym == "quasiquote" => {
                        eval_quasiquote(rest_forms.to_vec(), loop_env)
                    }
                    Symbol(ref sym) if sym == "try*" => eval_try(rest_forms.to_vec(), loop_env),
                    Symbol(ref sym) if sym == "macroexpand" => {
                        if !rest_forms.is_empty() {
                            macroexpand(&rest_forms[0], loop_env)
                        } else {
                            Err("macroexpand requires an argument".to_string())
                        }
                    }
                    RustFunction(f) => {
                        if let List(rest_evaled) =
                            eval_ast(&List(Rc::new(rest_forms.to_vec())), loop_env)?
                        {
                            f(rest_evaled.to_vec())
                        } else {
                            panic!("eval_ast List -> non-List")
                        }
                    }
                    RustClosure(c) => match rest_forms.get(0) {
                        Some(arg) => {
                            let abc = c.0;
                            (abc)(
                                EVAL(arg.clone(), loop_env.clone())?,
                                Env::get_env_top_level(loop_env),
                            )
                        }
                        None => return Err("argument required".to_string()),
                    },
                    func @ FnFunction {
                        is_macro: false, ..
                    } => apply_fnfunction(
                        func.clone(),
                        eval_ast(&list_from_vec(rest_forms), loop_env)?.clone(),
                        true,
                    ),
                    Symbol(_) | List(_) => match EVAL(form0, loop_env.clone()) {
                        Ok(List(ref x)) if x.is_empty() => {
                            Err("Cannot apply empty list as function".to_string())
                        }
                        Ok(form0_evaled) => {
                            let mut spliced_ast = vec![form0_evaled];
                            spliced_ast.append(&mut (&forms[1..]).to_vec());
                            Ok(Tco(Box::new(List(Rc::new(spliced_ast))), loop_env))
                        }
                        err @ Err(_) => return err,
                    },
                    other => Err(format!(
                        "not a symbol, list, or function: {}",
                        pr_str(&other, true)
                    )),
                };
                match loop_result {
                    Ok(Tco(exp, env)) => {
                        loop_env = env;
                        ast = *exp;
                        debug!(
                            "== EVAL {} looping with: {}",
                            pr_str(&original_ast, true),
                            pr_str(&ast.clone(), true)
                        );
                        continue 'tco;
                    }
                    x => {
                        debug!(
                            "<< EVAL {} returning: {}",
                            pr_str(&original_ast, true),
                            pr_str(&x.clone().unwrap(), true)
                        );
                        return x;
                    }
                }
            }
            Symbol(_)
            | Int(_)
            | MalExpression::String(_)
            | Vector(_)
            | HashTable(_)
            | Boolean(_)
            | FnFunction { .. }
            | Atom(_)
            | RustFunction(_)
            | RustClosure(_)
            | Nil() => {
                let result = eval_ast(&ast, loop_env);
                debug!(
                    "<< EVAL {} returning: eval_ast({}) = {}",
                    pr_str(&original_ast, true),
                    pr_str(&ast, true),
                    pr_str(&result.clone().unwrap(), true)
                );
                return result;
            }
        }
    }
}

fn eval_let(forms: Vec<MalExpression>, env: Rc<Env>) -> MalRet {
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

fn eval_do(forms: Vec<MalExpression>, env: Rc<Env>) -> MalRet {
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

fn eval_if(forms: Vec<MalExpression>, env: Rc<Env>) -> MalRet {
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

fn eval_def(forms: Vec<MalExpression>, env: Rc<Env>) -> MalRet {
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

fn eval_defmacro(forms: Vec<MalExpression>, env: Rc<Env>) -> MalRet {
    match (forms.get(0), forms.get(1)) {
        (Some(Symbol(f0)), Some(f1)) => {
            let key = f0;
            let value = EVAL(f1.clone(), env.clone())?;
            if let FnFunction {
                binds,
                ast,
                outer_env,
                ..
            } = value
            {
                let macro_fn = FnFunction {
                    binds,
                    ast,
                    outer_env,
                    is_macro: true,
                    closure: None,
                };
                env.set(key, macro_fn.clone());
                Ok(macro_fn)
            } else {
                Err("defmacro! requires second argument to evaluate to a function".to_string())
            }
        }
        _ => Err("defmacro! requires 2 arguments; first argument should be a symbol".to_string()),
    }
}

fn macroexpand_once(ast: &MalExpression, env: Rc<Env>) -> Option<MalRet> {
    match ast {
        List(l) => match l.get(0) {
            Some(Symbol(s)) => {
                let target = env.get(s);
                match target {
                    Some(inner_ast) => match inner_ast {
                        func @ FnFunction { is_macro: true, .. } => Some(apply_fnfunction(
                            func,
                            list_from_vec(l[1..].to_vec()),
                            false,
                        )),
                        _ => None,
                    },
                    _ => None,
                }
            }
            _ => None,
        },
        _ => None,
    }
}

fn macroexpand(ast: &MalExpression, env: Rc<Env>) -> MalRet {
    let mut ast = ast.clone();
    loop {
        match macroexpand_once(&ast, env.clone()) {
            Some(Ok(x)) => {
                ast = x;
                continue;
            }
            Some(y) => {
                trace!("macroexpand({:?}) = {:?}", ast, y);
                return y;
            }
            None => {
                trace!("macroexpand({:?}) = {:?}", ast, ast);
                return Ok(ast);
            }
        }
    }
}

fn eval_swap(forms: Vec<MalExpression>, env: Rc<Env>) -> MalRet {
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

fn eval_fn(forms: Vec<MalExpression>, env: Rc<Env>) -> MalRet {
    match (forms.get(0), forms.get(1)) {
        (Some(List(f0_v)), Some(f1)) | (Some(Vector(f0_v)), Some(f1)) => Ok(FnFunction {
            binds: f0_v.clone(),
            ast: Rc::new(f1.clone()),
            outer_env: env,
            is_macro: false,
            closure: Some(apply_fnfunction),
        }),
        _ => Err(
            "fn* expression must have at least two arguments; first must be list or vector"
                .to_string(),
        ),
    }
}

fn eval_quote(forms: Vec<MalExpression>, _env: Rc<Env>) -> MalRet {
    match forms.get(0) {
        Some(x) => Ok(x.clone()),
        None => Err("quote requires an argument".to_string()),
    }
}

fn eval_quasiquote(forms: Vec<MalExpression>, env: Rc<Env>) -> MalRet {
    let result = eval_quasiquote_inner(forms, env.clone());
    Ok(Tco(Box::new(result?), env))
}

fn eval_quasiquote_inner(forms: Vec<MalExpression>, env: Rc<Env>) -> MalRet {
    if log_enabled!(Level::Debug) {
        if let Some(x) = forms.get(0) {
            debug!("handle_quasiquote_inner: {}", pr_str(x, true));
        } else {
            debug!("handle_quasiquote_inner: <nothing>");
        }
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
                    let concat_2 = eval_quasiquote_inner(
                        vec![List(Rc::new(list_contents[1..].to_vec()))],
                        env,
                    )?;
                    Ok(List(Rc::new(vec![concat, concat_1, concat_2])))
                }
                (Some(Symbol(s)), None) if s == "splice-unquote" => {
                    Err("splice-unquote requires an argument".to_string())
                }
                _ => eval_quasiquote_inner_default_case_cons(list_contents, env),
            },
            _ => eval_quasiquote_inner_default_case_cons(list_contents, env),
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

fn eval_quasiquote_inner_default_case_cons(
    list_contents: &Rc<Vec<MalExpression>>,
    env: Rc<Env>,
) -> MalRet {
    let quasi_first = eval_quasiquote_inner(vec![list_contents[0].clone()], env.clone())?;
    let quasi_rest = eval_quasiquote_inner(vec![List(Rc::new(list_contents[1..].to_vec()))], env)?;
    Ok(List(Rc::new(vec![
        Symbol("cons".to_string()),
        quasi_first,
        quasi_rest,
    ])))
}

fn eval_try(forms: Vec<MalExpression>, env: Rc<Env>) -> MalRet {
    match (forms.get(0), forms.get(1)) {
        (Some(a0), Some(List(list_a1))) if list_a1.len() == 3 => {
            match (list_a1[0].clone(), list_a1[1].clone()) {
                (Symbol(ref catch), Symbol(ref catch_sym)) if catch.eq("catch*") => {
                    match EVAL(a0.clone(), env.clone()) {
                        result @ Ok(_) => result,
                        Err(error) => {
                            let catch_env = Env::simple_new(Some(env));
                            catch_env.set(&catch_sym, MalExpression::String(error));
                            EVAL(list_a1[2].clone(), Rc::new(catch_env))
                        }
                    }
                }
                _ => Err(format!(
                    "invalid catch* clause: {}",
                    printer::pr_str(&List(list_a1.clone()), true)
                )),
            }
        }
        _ => Err(format!(
            "invalid try*/catch* form: {}",
            printer::pr_str(forms.get(0).unwrap_or(&Nil()), true)
        )),
    }
}

fn apply_fnfunction(function: MalExpression, rest_forms: MalExpression, tco: bool) -> MalRet {
    match (function.clone(), rest_forms.clone()) {
        (
            FnFunction {
                binds,
                ast: fn_ast,
                outer_env,
                ..
            },
            List(rest_forms_vec),
        ) => {
            let binds_vec_string: Vec<String> = binds
                .iter()
                .map(|x| match x {
                    MalExpression::Symbol(x_symbol) => x_symbol.clone(),
                    _ => panic!("non-symbol {} in FnFunction binds", pr_str(x, true)),
                })
                .collect();
            let fn_env = Env::new(Some(outer_env), Rc::new(binds_vec_string), rest_forms_vec)?;
            if tco {
                Ok(Tco(Box::new((*fn_ast).clone()), Rc::new(fn_env)))
            } else {
                EVAL((*fn_ast).clone(), Rc::new(fn_env))
            }
        }
        (_, _) => panic!(
            "apply_fnfunction called with invalid arguments {:?} {:?}",
            function, rest_forms
        ),
    }
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
                None => Err(format!("'{}' not found", symbol)),
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
        Tco(_, _) => panic!("Tco not expected in eval_ast"),
        Int(_)
        | MalExpression::String(_)
        | Boolean(_)
        | FnFunction { .. }
        | Atom(_)
        | RustFunction(_)
        | RustClosure(_)
        | Nil() => Ok(ast.clone()),
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
        r#"(do (def! not (fn* (a) (if a false true)))
                    (def! load-file (fn* (f) (eval (read-string (str "(do " (slurp f) "\nnil)")))))
                    (defmacro! cond (fn* (& xs) (if (> (count xs) 0) (list 'if (first xs) (if (> (count xs) 1) (nth xs 1) (throw "odd number of forms to cond")) (cons 'cond (rest (rest xs))))))))"#,
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
    fn test_apply() {
        let env = Rc::new(core::core_ns());
        assert_eq!(rep("(apply + '(1 2))", env), Ok("3".to_string()));
    }

    #[test]
    fn test_eval() {
        let env = Rc::new(core::core_ns());
        assert_eq!(
            rep(
                "(do (def! nums (list 1 2 3)) (def! double (fn* (a) (* 2 a))) (map double nums))",
                env
            ),
            Ok("3".to_string())
        );
    }
}
