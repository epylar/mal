extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate rustyline;
extern crate itertools;

mod printer;
mod reader;
mod types;

use printer::pr_str;
use reader::read_str;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use types::MalExpression;
use std::collections::HashMap;

type Env = HashMap<String, MalExpression>;

#[allow(non_snake_case)]
fn READ(input: &str) -> Result<MalExpression, String> {
    read_str(input)
}

#[allow(non_snake_case)]
fn EVAL(ast: MalExpression, env: &Env) -> Result<MalExpression, String> {
    match ast.clone() {
        MalExpression::List(l) => {
            if l.is_empty() {
                Ok(ast)
            } else {
                let evaled = eval_ast(ast, env)?;
                match evaled {
                    MalExpression::List(el) => {
                        if el.is_empty() {
                            Err("internal error: eval_ast non empty list => empty list".to_string())
                        } else {
                            if let Some(MalExpression::Function(f)) = el.first() {
                                let rest = &el[1..];
                                f(MalExpression::List(rest.to_vec()))
                            } else {
                                match l.first() {
                                    Some(me) => {
                                        Err(format!("not a function: {}", pr_str(me)))
                                    },
                                    None => Err(format!("internal error: non-empty list has no first"))
                                }

                            }
                        }
                    },
                    _ => Err("internal error: eval_ast list => not list".to_string())
                }
            }
        },
        _ => {
            eval_ast(ast, env)
        }
    }
}

fn eval_ast(ast: MalExpression, env: &Env) -> Result<MalExpression, String> {
    match ast {
        MalExpression::Symbol(symbol) => {
            let get = env.get(&symbol);
            match get {
                Some(result) => Ok(result.clone()),
                None => Err(format!("symbol {} not found in environment", symbol))
            }
        },
        MalExpression::List(list) => {
            match list.into_iter().map(|x| EVAL(x, env)).collect() {
                Ok(collected) => Ok(MalExpression::List(collected)),
                Err(e) => Err(e)
            }
        },
        MalExpression::Vector(vector) => {
            match vector.into_iter().map(|x| EVAL(x, env)).collect() {
                Ok(collected) => Ok(MalExpression::Vector(collected)),
                Err(e) => Err(e)
            }
        },
        MalExpression::HashTable(hash_table) => {
            match hash_table.into_iter().map(|x| EVAL(x, env)).collect() {
                Ok(collected) => Ok(MalExpression::HashTable(collected)),
                Err(e) => Err(e)
            }
        },
        _ => Ok(ast)
    }
}

#[allow(non_snake_case)]
fn PRINT(form: Result<MalExpression, String>) -> Result<String, String> {
    Ok(pr_str(&form?))
}

fn rep(line: &str, env: &Env) -> Result<String, String> {
    PRINT(EVAL(READ(line)?, env))
}

fn plus(args: MalExpression) -> Result<MalExpression, String> {
    mal_int_fn(args, |a, b| a + b, 0)
}

fn minus(args: MalExpression) -> Result<MalExpression, String> {
    mal_int_fn_binary(args, |a, b| a - b)
}

fn times(args: MalExpression) -> Result<MalExpression, String> {
    mal_int_fn(args, |a, b| a * b, 1)
}

fn int_divide(args: MalExpression) -> Result<MalExpression, String> {
    mal_int_fn_binary(args, |a, b| a / b)
}

fn mal_int_fn_binary(args: MalExpression, func: fn(i32, i32) -> i32) -> Result<MalExpression, String> {
    if let MalExpression::List(l) = args {
        match (&l[0], &l[1]) {
            (MalExpression::Int(a), MalExpression::Int(b)) => Ok(MalExpression::Int(func(*a, *b))),
            _ => Err("invalid arguments to binary int function".to_string())
        }

    } else {
        Err("function called with non-list".to_string())
    }
}

fn mal_int_fn(args: MalExpression, func: fn(i32, i32) -> i32, initial: i32) -> Result<MalExpression, String> {
    if let MalExpression::List(l) = args {
        let mut result = initial;
        for x in l {
            match x {
                MalExpression::Int(x_int) => result = func(result, x_int),
                _ => return Err("function called with non-int".to_string())
            }
        }
        Ok(MalExpression::Int(result))
    } else {
        Err("function called with non-list".to_string())
    }
}

fn init_env() -> Env {
    let mut env: Env = HashMap::new();

    env.insert("+".to_string(), MalExpression::Function(|args| plus(args)));
    env.insert("-".to_string(), MalExpression::Function(|args| minus(args)));
    env.insert("*".to_string(), MalExpression::Function(|args| times(args)));
    env.insert("/".to_string(), MalExpression::Function(|args| int_divide(args)));

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
