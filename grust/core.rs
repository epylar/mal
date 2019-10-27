use crate::env::Env;
use crate::types::MalExpression;
use crate::types::MalExpression::{Int, RustFunction};
use crate::types::MalRet;
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
        !(self.is_nil() || self.is_empty_string() || self.is_zero() || self.is_false())
    }

    fn is_empty_string(&self) -> bool {
        match self {
            MalExpression::String(x) if x == "" => true,
            _ => false,
        }
    }

    fn is_zero(&self) -> bool {
        match self {
            MalExpression::Int(0) => true,
            _ => false,
        }
    }
}

pub fn core_ns() -> Env {
    fn plus(args: Vec<MalExpression>) -> MalRet {
        mal_int_fn(args, |a, b| a + b, 0)
    }

    fn minus(args: Vec<MalExpression>) -> MalRet {
        mal_int_fn_binary(args, |a, b| a - b)
    }

    fn times(args: Vec<MalExpression>) -> MalRet {
        mal_int_fn(args, |a, b| a * b, 1)
    }

    fn int_divide(args: Vec<MalExpression>) -> MalRet {
        mal_int_fn_binary(args, |a, b| a / b)
    }

    fn mal_int_fn_binary(args: Vec<MalExpression>, func: fn(i32, i32) -> i32) -> MalRet {
        match (args.get(0), args.get(1)) {
            (Some(Int(a)), Some(Int(b))) => Ok(Int(func(*a, *b))),
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

    let mut env = Env::new(None, Rc::new(vec![]), Rc::new(vec![]));

    env.set("+", RustFunction(plus));
    env.set("-", RustFunction(minus));
    env.set("*", RustFunction(times));
    env.set("/", RustFunction(int_divide));

    env
}
