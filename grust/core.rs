use crate::env::Env;
use crate::printer::pr_str;
use crate::reader::read_str;
use crate::types::MalExpression;
use crate::types::MalExpression::{
    Atom, Boolean, HashTable, Int, List, Nil, RustFunction, Symbol, Vector,
};
use crate::types::MalRet;
use itertools::Itertools;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

impl MalExpression {
    fn is_nil(&self) -> bool {
        if let MalExpression::Nil() = self {
            true
        } else {
            false
        }
    }

    pub fn is_false(&self) -> bool {
        match self {
            MalExpression::Boolean(x) if x == &false => true,
            _ => false,
        }
    }

    pub fn is_true_in_if(&self) -> bool {
        !(self.is_nil() || self.is_false())
    }

    fn equals(&self, other: &MalExpression) -> bool {
        fn compare_vecs(a: &Rc<Vec<MalExpression>>, b: &Rc<Vec<MalExpression>>) -> bool {
            if a.len() != b.len() {
                return false;
            }
            for i in 0..(a.len()) {
                if !&a[i].equals(&b[i]) {
                    return false;
                }
            }
            true
        }

        match self {
            Symbol(x) => match other {
                Symbol(y) => x == y,
                _ => false,
            },
            Int(x) => match other {
                Int(y) => x == y,
                _ => false,
            },
            MalExpression::String(x) => match other {
                MalExpression::String(y) => x == y,
                _ => false,
            },
            Boolean(x) => match other {
                Boolean(y) => x == y,
                _ => false,
            },
            List(x) | Vector(x) => match other {
                List(y) | Vector(y) => compare_vecs(x, y),
                _ => false,
            },
            HashTable(x) => match other {
                HashTable(y) => compare_vecs(x, y),
                _ => false,
            },
            Nil() => match other {
                Nil() => true,
                _ => false,
            },
            _ => false,
        }
    }
}

pub fn core_ns() -> Env {
    fn plus(args: Vec<MalExpression>) -> MalRet {
        mal_int_fn(args, |a, b| a + b, 0)
    }

    fn minus(args: Vec<MalExpression>) -> MalRet {
        mal_int_fn_binary_int(args, |a, b| a - b)
    }

    fn times(args: Vec<MalExpression>) -> MalRet {
        mal_int_fn(args, |a, b| a * b, 1)
    }

    fn int_divide(args: Vec<MalExpression>) -> MalRet {
        mal_int_fn_binary_int(args, |a, b| a / b)
    }

    fn less_than(args: Vec<MalExpression>) -> MalRet {
        mal_int_fn_binary_bool(args, |a, b| a < b)
    }

    fn less_than_equals(args: Vec<MalExpression>) -> MalRet {
        mal_int_fn_binary_bool(args, |a, b| a <= b)
    }

    fn more_than(args: Vec<MalExpression>) -> MalRet {
        mal_int_fn_binary_bool(args, |a, b| a > b)
    }

    fn more_than_equals(args: Vec<MalExpression>) -> MalRet {
        mal_int_fn_binary_bool(args, |a, b| a >= b)
    }

    fn mal_int_fn_binary_int(args: Vec<MalExpression>, func: fn(i32, i32) -> i32) -> MalRet {
        match (args.get(0), args.get(1)) {
            (Some(Int(a)), Some(Int(b))) => Ok(Int(func(*a, *b))),
            _ => Err("invalid arguments to binary int function".to_string()),
        }
    }

    fn mal_int_fn_binary_bool(args: Vec<MalExpression>, func: fn(i32, i32) -> bool) -> MalRet {
        match (args.get(0), args.get(1)) {
            (Some(Int(a)), Some(Int(b))) => Ok(Boolean(func(*a, *b))),
            _ => Err("invalid arguments to binary int function".to_string()),
        }
    }

    fn mal_int_fn(args: Vec<MalExpression>, func: fn(i32, i32) -> i32, initial: i32) -> MalRet {
        let mut result = initial;
        for x in args {
            match x {
                Int(x_int) => result = func(result, x_int),
                _ => return Err("function called with non-int".to_string()),
            }
        }
        Ok(Int(result))
    }

    fn list(args: Vec<MalExpression>) -> MalRet {
        Ok(MalExpression::List(Rc::new(args)))
    }

    fn list_q(args: Vec<MalExpression>) -> MalRet {
        match args.get(0) {
            Some(List(_)) => Ok(Boolean(true)),
            None => Err("list? requires an argument".to_string()),
            _ => Ok(Boolean(false)),
        }
    }

    fn empty_q(args: Vec<MalExpression>) -> MalRet {
        match args.get(0) {
            Some(List(l)) | Some(Vector(l)) => Ok(Boolean(l.is_empty())),
            None => Err("empty? requires a list or vector argument".to_string()),
            _ => Ok(Boolean(false)),
        }
    }

    fn count(args: Vec<MalExpression>) -> MalRet {
        match args.get(0) {
            Some(List(l)) | Some(Vector(l)) => Ok(Int(l.len() as i32)),
            Some(Nil()) => Ok(Int(0)),
            _ => Err("count requires a list, vector, or nil argument".to_string()),
        }
    }

    fn equal(args: Vec<MalExpression>) -> MalRet {
        match (args.get(0), args.get(1)) {
            (Some(a), Some(b)) => Ok(Boolean(a.equals(b))),
            _ => Err("= requires two arguments".to_string()),
        }
    }

    fn pr_dash_str(args: Vec<MalExpression>) -> MalRet {
        Ok(MalExpression::String(
            args.iter().map(|x| pr_str(x, true)).join(" "),
        ))
    }

    fn str(args: Vec<MalExpression>) -> MalRet {
        Ok(MalExpression::String(
            args.iter().map(|x| pr_str(x, false)).join(""),
        ))
    }

    fn prn(args: Vec<MalExpression>) -> MalRet {
        println!("{}", args.iter().map(|x| pr_str(x, true)).join(" "));
        Ok(Nil())
    }

    fn println(args: Vec<MalExpression>) -> MalRet {
        println!("{}", args.iter().map(|x| pr_str(x, false)).join(" "));
        Ok(Nil())
    }

    fn read_dash_string(args: Vec<MalExpression>) -> MalRet {
        match args.get(0) {
            Some(MalExpression::String(s)) => read_str(s),
            _ => Err("read-string requires a string argument".to_string()),
        }
    }

    fn slurp(args: Vec<MalExpression>) -> MalRet {
        match args.get(0) {
            Some(MalExpression::String(filename)) => match fs::read_to_string(filename) {
                Ok(contents) => Ok(MalExpression::String(contents)),
                Err(e) => Err(format!("error reading file {}: {}", filename, e)),
            },
            _ => Err("read-string requires a string argument".to_string()),
        }
    }

    fn atom(args: Vec<MalExpression>) -> MalRet {
        match args.get(0) {
            Some(expression) => Ok(Atom(Rc::new(RefCell::new(expression.clone())))),
            None => Err("atom requires an argument".to_string()),
        }
    }

    fn atom_q(args: Vec<MalExpression>) -> MalRet {
        match args.get(0) {
            Some(Atom(_)) => Ok(Boolean(true)),
            None => Err("atom? requires an argument".to_string()),
            _ => Ok(Boolean(false)),
        }
    }

    let env = match Env::new(None, Rc::new(vec![]), Rc::new(vec![])) {
        Ok(e) => e,
        Err(e) => panic!("Error setting up initial environment: {}", e),
    };

    env.set("+", RustFunction(plus));
    env.set("-", RustFunction(minus));
    env.set("*", RustFunction(times));
    env.set("/", RustFunction(int_divide));
    env.set("<", RustFunction(less_than));
    env.set("<=", RustFunction(less_than_equals));
    env.set(">", RustFunction(more_than));
    env.set(">=", RustFunction(more_than_equals));
    env.set("list", RustFunction(list));
    env.set("list?", RustFunction(list_q));
    env.set("empty?", RustFunction(empty_q));
    env.set("count", RustFunction(count));
    env.set("=", RustFunction(equal));
    env.set("pr-str", RustFunction(pr_dash_str));
    env.set("str", RustFunction(str));
    env.set("prn", RustFunction(prn));
    env.set("println", RustFunction(println));
    env.set("read-string", RustFunction(read_dash_string));
    env.set("slurp", RustFunction(slurp));
    env.set("atom", RustFunction(atom));
    env.set("atom?", RustFunction(atom_q));

    env
}
