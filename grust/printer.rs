use crate::types::MalExpression;
use crate::types::MalExpression::{
    Boolean, FnFunction, HashTable, Int, List, Nil, RustClosure, RustFunction, Symbol, Vector,
};

pub fn pr_str(expression: &MalExpression, print_readably: bool) -> String {
    match expression {
        Int(i) => i.to_string(),
        Symbol(s) => s.to_string(),
        MalExpression::String(s) => {
            if s.starts_with("\u{29e}") {
                format!(":{}", &s[2..])
            } else if print_readably {
                "\"".to_owned()
                    + &s.replace("\\", "\\\\")
                        .replace("\"", "\\\"")
                        .replace("\n", "\\n")
                    + "\""
            } else {
                s.to_string()
            }
        }
        List(l) => {
            let middle: Vec<String> = l.iter().map(|x| pr_str(x, print_readably)).collect();
            format!("({})", middle.join(" "))
        }
        Vector(l) => {
            let middle: Vec<String> = l.iter().map(|x| pr_str(x, print_readably)).collect();
            format!("[{}]", middle.join(" "))
        }
        HashTable(l) => {
            let middle: Vec<String> = l.iter().map(|x| pr_str(x, print_readably)).collect();
            format!("{}{}{}", "{", middle.join(" "), "}")
        }
        RustFunction(_) => "#<Rust unction>".to_string(),
        FnFunction { binds, ast, .. } => format!(
            "#<fn* function: binds = {}; ast = {}>",
            pr_str(&Vector(binds.clone()), print_readably),
            pr_str(ast, print_readably),
        ),
        RustClosure(_) => "#<Rust closure>".to_string(),
        Boolean(x) => match x {
            true => "true".to_string(),
            false => "false".to_string(),
        },
        Nil() => "nil".to_string(),
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_pr_str() {
        assert_eq!(pr_str(&Int(1), true), "1");
    }
}
