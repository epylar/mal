use crate::env::Env;
use core::fmt;
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
    },
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
