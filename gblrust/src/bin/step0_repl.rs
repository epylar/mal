extern crate mal;
extern crate rustyline;
use rustyline::Editor;

fn read(a: String) -> String {
    a
}

fn eval(a: String) -> String {
    a
}

fn print(a: String) -> String {
    a
}

fn rep(a: String) -> String {
    print(eval(read(a)))
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

        print!("{}", rep(input))
    }
}

