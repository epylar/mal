use types::MalExpression;
use reader::read_str;

fn pr_str(expression: &MalExpression) -> String {
    match expression {
        MalExpression::Int(i) => i.to_string(),
        MalExpression::Symbol(s) => s.to_string(),
        MalExpression::String(s) => s.to_string(),
        MalExpression::List(l) => {
            "list".to_string()
        }
        _ => "not_implemented".to_string()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_pr_str() {
        assert_eq!(pr_str(&MalExpression::Int(1)), "1");
        assert_eq!(pr_str(&read_str("1").unwrap()), "1");
        assert_eq!(pr_str(&read_str("a").unwrap()), "a");
        assert_eq!(pr_str(&read_str("\"a\"").unwrap()), "\"a\"");
    }
}
