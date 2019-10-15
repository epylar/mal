use types::MalExpression;

pub(crate) fn pr_str(expression: &MalExpression) -> String {
    match expression {
        MalExpression::Int(i) => i.to_string(),
        MalExpression::Symbol(s) => s.to_string(),
        MalExpression::String(s) => s.to_string(),
        MalExpression::List(l) => {
            let middle: Vec<String> = l.iter().map(pr_str).collect();
            format!("({})", middle.join(" "))
        }
        MalExpression::Vector(l) => {
            let middle: Vec<String> = l.iter().map(pr_str).collect();
            format!("[{}]", middle.join(" "))
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_pr_str() {
        assert_eq!(pr_str(&MalExpression::Int(1)), "1");
    }
}
