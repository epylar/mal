extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use types::MalExpression;
use reader::read_str;
use printer::pr_str;

mod reader;
mod printer;
mod types;

fn READ(input: &str) -> Option<MalExpression> {
    read_str(input)
}

fn EVAL(form: Option<MalExpression>) -> Option<MalExpression> {
    form
}

fn PRINT(form: Option<MalExpression>) -> Option<String> {
    match form {
        Some(actual_form) => Some(pr_str(&actual_form)),
        None => None
    }
}

fn rep(line: &str) -> Option<String> {
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
                if line.len() > 0 {
                    println!("{}", rep(&line.to_owned()));
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
