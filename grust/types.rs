use crate::env::Env;
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
        outer_env: Env,
    },
    RustFunction(fn(Vec<MalExpression>) -> MalRet),
    Nil(),
}

pub type MalRet = Result<MalExpression, String>;

pub fn iterate_rc_vec(data: Rc<Vec<MalExpression>>) -> impl Iterator<Item = MalExpression> {
    let len = data.len();
    (0..len).map(move |i| data[i].clone())
}
