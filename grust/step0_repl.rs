extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

#[allow(non_snake_case)]
fn READ(input: &str) -> &str {
    input
}

#[allow(non_snake_case)]
fn EVAL(form: &str) -> &str {
    form
}

#[allow(non_snake_case)]
fn PRINT(form: &str) -> &str {
    form
}

fn rep(line: &str) -> &str {
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
                    println!("{}", rep(&line.as_ref()));
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
