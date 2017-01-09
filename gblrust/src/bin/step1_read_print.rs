extern crate mal;
extern crate rustyline;
use mal::types::MalType;
use mal::reader;
use rustyline::Editor;

fn read(a: &str) -> Result<MalType, String> {
    reader::read_str(a)
}

// EVAL continues to simply return its input but the type is now a mal data type.
fn eval(x: MalType) -> MalType {    
    x
}

fn print(x: MalType) -> String {
    // and the PRINT function to call printer.pr_str.
    x.pr_str()
}

fn rep(a: &str) -> Result<String, String> {
    match read(a) {
        Ok(x) => Ok(print(eval(x))),
        Err(y) => Err(y)
    }
}

fn main() {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("user> ");
        let input = match readline {
            Ok(input) => input,
            Err(y) => {
                println!();
                break;
            },
        };
        
        match rep(&(input.to_owned()[..])) {
            Ok(x) => println!("{}", x),
            Err(y) => println!("{}", y)
        }
    }
}
