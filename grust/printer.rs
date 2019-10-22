use crate::types::MalExpression;
use crate::types::MalExpression::{Function, HashTable, Int, List, Symbol, Vector};

pub fn pr_str(expression: &MalExpression) -> String {
    match expression {
        Int(i) => i.to_string(),
        Symbol(s) => s.to_string(),
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
        }
        List(l) => {
            let middle: Vec<String> = l.iter().map(pr_str).collect();
            format!("({})", middle.join(" "))
        }
        Vector(l) => {
            let middle: Vec<String> = l.iter().map(pr_str).collect();
            format!("[{}]", middle.join(" "))
        }
        HashTable(l) => {
            let middle: Vec<String> = l.iter().map(pr_str).collect();
            format!("{}{}{}", "{", middle.join(" "), "}")
        }
        Function(_) => "<function>".to_string(),
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_pr_str() {
        assert_eq!(pr_str(&Int(1)), "1");
    }
}
