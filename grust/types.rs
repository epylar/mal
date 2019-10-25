use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum MalExpression {
    Symbol(String),
    Int(i32),
    List(Rc<Vec<MalExpression>>),
    String(String),
    Vector(Rc<Vec<MalExpression>>),
    HashTable(Rc<Vec<MalExpression>>),
    Function(fn(MalExpression) -> MalRet),
    Nil()
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

    pub fn is_true_ish(&self) -> bool {
        !(self.is_nil() || self.is_empty_string() || self.is_zero())
    }

    fn is_empty_string(&self) -> bool {
        match self {
            MalExpression::String(x) if x == "" => true,
            _ => false
        }
    }

    fn is_zero(&self) -> bool {
        match self {
            MalExpression::Int(0) => true,
            _ => false
        }
    }
}
