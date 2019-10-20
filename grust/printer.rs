use types::MalExpression;

pub fn pr_str(expression: &MalExpression) -> String {
    match expression {
        MalExpression::Int(i) => i.to_string(),
        MalExpression::Symbol(s) => s.to_string(),
        MalExpression::String(s) => {
            if s.starts_with("\u{29e}") {
                format!(":{}", &s[2..])
            } else {
                "\"".to_owned()
                    + &s.replace("\\", "\\\\")
                    .replace("\"", "\\\"")
                    .replace("\n", "\\n")
                    + "\""
            }
        },
        MalExpression::List(l) => {
            let middle: Vec<String> = l.iter().map(pr_str).collect();
            format!("({})", middle.join(" "))
        },
        MalExpression::Vector(l) => {
            let middle: Vec<String> = l.iter().map(pr_str).collect();
            format!("[{}]", middle.join(" "))
        },
        MalExpression::HashTable(l) => {
            let middle: Vec<String> = l.iter().map(pr_str).collect();
            format!("{}{}{}", "{", middle.join(" "), "}")
        },
        MalExpression::Function(_) => {
            format!("<function>")
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
