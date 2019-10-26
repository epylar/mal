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
    RustFunction(fn(MalExpression) -> MalRet),
    Nil(),
}

pub type MalRet = Result<MalExpression, String>;

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
