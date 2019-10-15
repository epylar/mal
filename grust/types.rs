
#[derive(Debug)]
pub(crate) enum MalExpression {
    Symbol(String),
    Int(u32),
    List(Vec<MalExpression>)
}