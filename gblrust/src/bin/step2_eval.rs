extern crate mal;
extern crate rustyline;
use mal::types::MalType::*;
use mal::types::MalVal;
use mal::reader;
use std::rc::Rc;
use std::collections::HashMap;
use mal::types::MalRet;
use rustyline::Editor;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

fn read(a: &str) -> MalRet {
    reader::read_str(a).map(|i| Rc::new(i))
}


fn mal_plus(items: Vec<MalVal>) -> MalRet {
    let mut sum = 0;
    for item in items {
        match *item {
            Integer(x) => sum += x,
            _ => return Err(format!("can only add integers but saw {}", item.pr_str())),
        }
    }
    Ok(Rc::new(Integer(sum)))
}

fn mal_minus(items: Vec<MalVal>) -> MalRet {
    if items.len() != 2 {
        Err("too many items for -".to_string())
    } else {
        if let Integer(x) = *(items[0]) {
            if let Integer(y) = *(items[1]) {
                return Ok(Rc::new(Integer(x - y)));
            }
        }
        Err("need integer types for -".to_string())
    }
}

fn mal_times(items: Vec<MalVal>) -> MalRet {
    let mut product = 1;
    for item in items {
        match *item {
            Integer(x) => product *= x,
            _ => return Err(format!("can only multiply integers but saw {:?}", item.pr_str())),
        }
    }
    Ok(Rc::new(Integer(product)))
}

fn mal_divide(items: Vec<MalVal>) -> MalRet {
    if items.len() != 2 {
        Err("too many items for /".to_owned())
    } else {
        if let Integer(x) = *(items[0]) {
            if let Integer(y) = *(items[1]) {
                return Ok(Rc::new(Integer(x / y)));
            }
        }
        Err("need integer types for /".to_owned())
    }
}

/// Evaluates a Mal AST. This means that...
///
/// # Examples
/// ```
/// ...TBD...
/// ```
fn eval_ast(ast: MalVal, env: &HashMap<String, MalVal>) -> MalRet {
    //println!("eval_ast to {} and env", ast.pr_str());
    match *ast {
        // we simply look up symbols
        Symbol(ref s) => Ok(try!(lookup(env, s))),
        List(ref x) => {
            // we're wanting to map eval over everything in the list
            let mut new_list: Vec<MalVal> = vec![];
            for item in x {
                let result = try!(eval(item.clone(), env));
                new_list.push(result)
            }
            Ok(Rc::new(List(new_list)))
        }
        Vector(_) => Err("vector unimplemented in eval_ast".to_string()),
        // -> when we have Hash.. Hash(_) => unimplemented!(),
        _ => Ok(ast.clone()),
    }
}



fn eval(ast: MalVal, env: &HashMap<String,MalVal>) -> MalRet {
    // if we have a list: eval_ast every element and then apply
    // the function of the first element to the rest of the elements
    // otherwise, simple eval_ast the whole thing
    match *ast {
        List(_) => (),  // continue
        _ => return eval_ast(ast, env),
    }


    // apply list
    match eval_ast(ast, env) {
        Ok(ok) => {
            //println!("apply list ast: {}, env", ok.pr_str());
            match *ok {
                List(ref args) => {
                    let ref f = args.clone()[0];
                    f.apply(args[1..].to_vec())
                }
                _ => Err("Expected list".to_string()),
            }
        }
        Err(e) => Err(e),
    }
}

fn lookup(env: &HashMap<String, MalVal>, thing: &String) -> MalRet {
    let x: Option<&MalVal> = env.get(thing);
    match x {
        Some(y) => Ok(y.clone()),
        None => Err("abc".to_string())
    }
}

fn print(x: MalVal) -> String {
    // and the PRINT function to call printer.pr_str.
    x.pr_str()
}

fn rep(a: &str) -> Result<String, String> {
    let mut repl_env: HashMap<String, MalVal> = HashMap::new();
    repl_env.insert("+".to_string(), Rc::new(Func(mal_plus)));
    repl_env.insert("-".to_string(), Rc::new(Func(mal_minus)));
    repl_env.insert("*".to_string(), Rc::new(Func(mal_times)));
    repl_env.insert("/".to_string(), Rc::new(Func(mal_divide)));
    match read(a) {
        Ok(ast) => {
            let evaled = eval(ast, &repl_env);
            match evaled {
                Ok(x) => Ok(print(x)),
                Err(y) => Err(y),
            }
        }
        Err(y) => Err(y),
    }
}

fn main() {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("user> ");
        let input = match readline {
            Ok(input) => input,
            Err(y) => {
                println!("");
                break;
            }
        };

        match rep(&(input.to_owned()[..])) {
            Ok(x) => println!("{}", x),
            Err(y) => println!("{}", y),
        }
    }
}
