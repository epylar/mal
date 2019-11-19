use crate::env::Env;
use crate::types::MalExpression::{
    Atom, Boolean, FnFunction, HashTable, Int, List, Nil, RustClosure, RustFunction, Symbol, Tco,
    Vector,
};
use core::fmt;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum MalExpression {
    Symbol(String),
    Int(i32),
    List(Rc<Vec<MalExpression>>),
    String(String),
    Vector(Rc<Vec<MalExpression>>),
    HashTable(Rc<Vec<MalExpression>>),
    Boolean(bool),
    FnFunction {
        binds: Rc<Vec<MalExpression>>,
        ast: Rc<MalExpression>,
        outer_env: Rc<Env>,
        is_macro: bool,
        closure: Option<fn(MalExpression, MalExpression, bool) -> MalRet>, // fn itself and evaled args; tco? true or false
    },
    Atom(Rc<RefCell<MalExpression>>),
    Tco(Box<MalExpression>, Rc<Env>), // for tail call optimization; loop again in EVAL
    RustFunction(fn(Vec<MalExpression>) -> MalRet),
    RustClosure(Closure),
    Nil(),
}

#[derive(Clone)]
pub struct Closure(pub Rc<dyn Fn(MalExpression, Rc<Env>) -> MalRet>);

impl fmt::Debug for Closure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#<Rust closure>")
    }
}

pub type MalRet = Result<MalExpression, String>;

pub fn iterate_rc_vec(data: Rc<Vec<MalExpression>>) -> impl Iterator<Item = MalExpression> {
    let len = data.len();
    (0..len).map(move |i| data[i].clone())
}

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

    pub fn equals(&self, other: &MalExpression) -> bool {
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
