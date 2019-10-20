#[derive(Debug, Clone)]
pub enum MalExpression {
    Symbol(String),
    Int(i32),
    List(Vec<MalExpression>),
    String(String),
    Vector(Vec<MalExpression>),
    HashTable(Vec<MalExpression>),
    Function(fn(MalExpression) -> MalRet)
}

pub type MalRet = Result<MalExpression, String>;
