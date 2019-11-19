use crate::env::Env;
use crate::printer::pr_str;
use crate::reader::read_str;
use crate::types::MalExpression;
use crate::types::MalExpression::{
    Atom, Boolean, FnFunction, HashTable, Int, List, Nil, RustClosure, RustFunction, Symbol, Tco,
    Vector,
};
use crate::types::MalRet;
use crate::{printer, types};
use itertools::Itertools;
use std::cell::RefCell;
use std::convert::TryInto;
use std::rc::Rc;
use std::{fs, iter};

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

    pub fn first(&self) -> Option<&MalExpression> {
        match self {
            List(l) => l.get(0),
            _ => None,
        }
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
            FnFunction { .. } => false,
            Atom(_) => false,
            Tco(_, _) => false,
            RustFunction(_) => false,
            RustClosure(_) => false,
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

    fn deref(args: Vec<MalExpression>) -> MalRet {
        match args.get(0) {
            Some(Atom(a)) => Ok(a.borrow().clone()),
            None => Err("deref requires an argument".to_string()),
            _ => Ok(Boolean(false)),
        }
    }

    fn reset(args: Vec<MalExpression>) -> MalRet {
        match (args.get(0), args.get(1)) {
            (Some(Atom(a)), Some(b)) => {
                a.replace(b.clone());
                Ok(b.clone())
            }
            _ => Err("reset! requires two arguments: atom, new atom contents".to_string()),
        }
    }

    fn cons(args: Vec<MalExpression>) -> MalRet {
        match (args.get(0), args.get(1)) {
            (Some(a), Some(List(b))) | (Some(a), Some(Vector(b))) => {
                let cons_vec = iter::once(a).chain(b.iter()).cloned().collect_vec();
                Ok(List(Rc::new(cons_vec)))
            }
            _ => Err("cons requires two arguments: second must be a list or vector".to_string()),
        }
    }

    fn concat(args: Vec<MalExpression>) -> MalRet {
        let flat: Result<Vec<_>, _> = args
            .iter()
            .map(|x: &MalExpression| match x.clone() {
                List(l) | Vector(l) => Ok((*l).clone()),
                _ => Err(format!("concat: {} not a list or vector", pr_str(x, true))),
            })
            .collect();
        let flat: Vec<Vec<MalExpression>> = flat?;
        let flat: Vec<MalExpression> = flat.iter().flatten().cloned().collect();
        Ok(List(Rc::new(flat)))
    }

    let env = match Env::new(None, Rc::new(vec![]), Rc::new(vec![])) {
        Ok(e) => e,
        Err(e) => panic!("Error setting up initial environment: {}", e),
    };

    fn nth(args: Vec<MalExpression>) -> MalRet {
        match (args.get(0), args.get(1)) {
            (Some(List(l)), Some(Int(i))) | (Some(Vector(l)), Some(Int(i))) => {
                let index: usize = (*i).try_into().unwrap();
                if index < l.len() {
                    Ok(types::iterate_rc_vec(l.clone()).nth(index).unwrap())
                } else {
                    Err("nth called with out of range index".to_string())
                }
            }
            _ => Err(
                "nth requires two arguments: list/vector and integer index into list/vector"
                    .to_string(),
            ),
        }
    }

    fn first(args: Vec<MalExpression>) -> MalRet {
        match args.get(0) {
            Some(List(l)) | Some(Vector(l)) => match l.get(0) {
                Some(x) => Ok(x.clone()),
                None => Ok(Nil()),
            },
            Some(Nil()) => Ok(Nil()),
            _ => Err("first requires an argument".to_string()),
        }
    }

    fn rest(args: Vec<MalExpression>) -> MalRet {
        match args.get(0) {
            Some(List(l)) | Some(Vector(l)) => {
                if !l.is_empty() {
                    Ok(List(Rc::new(l[1..].to_vec())))
                } else {
                    Ok(List(Rc::new(vec![])))
                }
            }
            Some(Nil()) => Ok(List(Rc::new(vec![]))),
            Some(_) => Err("invalid argument to rest: must be a non-empty list/vector".to_string()),
            None => Err("rest requires an argument".to_string()),
        }
    }

    fn throw(args: Vec<MalExpression>) -> MalRet {
        match args.get(0) {
            Some(form) => Err(printer::pr_str(form, false)),
            None => Err("throw requires an argument".to_string()),
        }
    }

    fn collect_apply_eval_args(args: Vec<MalExpression>) -> Result<Vec<MalExpression>, String> {
        match args.get(args.len() - 1) {
            Some(List(x)) | Some(Vector(x)) => {
                let iter_a = args[1..(args.len() - 1)].iter();
                let iter_b = x.iter();
                let result: Vec<MalExpression> = iter_a.chain(iter_b).cloned().collect();
                Ok(result)
            }
            _ => Err("apply/eval functions require last argument to be a list".to_string()),
        }
    }

    fn apply(args: Vec<MalExpression>) -> MalRet {
        let apply_args_vec = match collect_apply_eval_args(args.clone()) {
            Ok(x) => x,
            Err(y) => return Err(y),
        };
        match args.get(0) {
            Some(func) => match func.clone() {
                FnFunction { closure, .. } => match closure {
                    Some(closure) => closure(func.clone(), List(Rc::new(apply_args_vec)), false),
                    None => panic!("Apply called with unimplemented FnFunction closure"),
                },
                RustFunction(x) => x(apply_args_vec),
                _ => Err("cannot apply with non-function".to_string()),
            },
            None => Err("apply requires arguments".to_string()),
        }
    }

    fn map(args: Vec<MalExpression>) -> MalRet {
        let map_args_vec = match collect_apply_eval_args(args.clone()) {
            Ok(x) => x,
            Err(y) => return Err(y),
        };
        match args.get(0) {
            Some(func) => match func.clone() {
                FnFunction { closure, .. } => match closure {
                    Some(closure) => {
                        let result: Result<Vec<MalExpression>, String> = map_args_vec
                            .into_iter()
                            .map(|x| closure(func.clone(), List(Rc::new(vec![x.clone()])), false))
                            .collect();
                        match result {
                            Ok(x) => Ok(List(Rc::new(x))),
                            Err(e) => Err(e),
                        }
                    }
                    None => panic!("Map called with unimplemented FnFunction closure"),
                },
                RustFunction(rsfunc) => {
                    let result: Result<Vec<MalExpression>, String> = map_args_vec
                        .into_iter()
                        .map(|x| rsfunc(vec![x.clone()]))
                        .collect();
                    match result {
                        Ok(x) => Ok(List(Rc::new(x))),
                        Err(e) => Err(e),
                    }
                }
                _ => Err("cannot map with non-function".to_string()),
            },
            None => Err("map requires arguments".to_string()),
        }
    }

    fn symbol_q(args: Vec<MalExpression>) -> MalRet {
        match args.get(0) {
            Some(Symbol(_)) => Ok(Boolean(true)),
            Some(_) => Ok(Boolean(false)),
            None => Err("symbol_q requires an argument".to_string()),
        }
    }

    fn nil_q(args: Vec<MalExpression>) -> MalRet {
        match args.get(0) {
            Some(Nil()) => Ok(Boolean(true)),
            Some(_) => Ok(Boolean(false)),
            None => Err("nil_q requires an argument".to_string()),
        }
    }

    fn true_q(args: Vec<MalExpression>) -> MalRet {
        match args.get(0) {
            Some(Boolean(true)) => Ok(Boolean(true)),
            Some(_) => Ok(Boolean(false)),
            None => Err("true_q requires an argument".to_string()),
        }
    }

    fn false_q(args: Vec<MalExpression>) -> MalRet {
        match args.get(0) {
            Some(Boolean(false)) => Ok(Boolean(true)),
            Some(_) => Ok(Boolean(false)),
            None => Err("false_q requires an argument".to_string()),
        }
    }

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
    env.set("deref", RustFunction(deref));
    env.set("reset!", RustFunction(reset));
    env.set("cons", RustFunction(cons));
    env.set("concat", RustFunction(concat));
    env.set("nth", RustFunction(nth));
    env.set("first", RustFunction(first));
    env.set("rest", RustFunction(rest));
    env.set("throw", RustFunction(throw));
    env.set("symbol?", RustFunction(symbol_q));
    env.set("nil?", RustFunction(nil_q));
    env.set("true?", RustFunction(true_q));
    env.set("false?", RustFunction(false_q));
    env.set("apply", RustFunction(apply));
    env.set("map", RustFunction(map));

    env
}
