extern crate mal;

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
    loop {
        let input = match mal::readline::readline("user> ") {
            Some(input) => input,
            None => {
                println!("");
                break;
            },
        };
        
        println!("{}", rep(input))
    }
}
