extern crate regex;
extern crate rustyline;

use printer::pr_str;
use reader::read_str;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use types::MalRet;

pub mod printer;
pub mod reader;
pub mod types;
pub mod env;

#[allow(non_snake_case)]
fn READ(input: &str) -> MalRet {
    read_str(input)
}

#[allow(non_snake_case)]
fn EVAL(form: MalRet) -> MalRet {
    form
}

#[allow(non_snake_case)]
fn PRINT(form: MalRet) -> Result<String, String> {
    match form {
        Ok(actual_form) => Ok(pr_str(&actual_form)),
        Err(e) => Err(e),
    }
}

fn rep(line: &str) -> Result<String, String> {
    PRINT(EVAL(READ(line)))
}

fn main() {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
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
                    match rep(&line.to_owned()) {
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
        assert_eq!(rep("(1 2 3)").unwrap(), "(1 2 3)");
        assert_eq!(
            rep("(1 2\r\n"),
            Err("EOF while reading sequence".to_string())
        );
        assert_eq!(rep("(1 \"a\" 2 3 (c))").unwrap(), "(1 \"a\" 2 3 (c))");
        assert_eq!(rep("1").unwrap(), "1");
        assert_eq!(rep("a").unwrap(), "a");
        assert_eq!(rep("\"a\"").unwrap(), "\"a\"");
        assert_eq!(rep("(1  2 3)").unwrap(), "(1 2 3)");
    }

    #[test]
    fn test_rep_nested_lists() {
        assert_eq!(rep("(()())").unwrap(), "(() ())")
    }
}
