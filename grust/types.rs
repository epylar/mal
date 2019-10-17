#[derive(Debug)]
pub(crate) enum MalExpression {
    Symbol(String),
    Int(i32),
    List(Vec<MalExpression>),
    String(String),
    Vector(Vec<MalExpression>),
    HashTable(Vec<MalExpression>)
}
