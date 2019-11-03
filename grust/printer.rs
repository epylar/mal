use crate::types::MalExpression;
use crate::types::MalExpression::{
    Atom, Boolean, FnFunction, HashTable, Int, List, Nil, RustClosure, RustFunction, Symbol, Tco,
    Vector,
};

pub fn pr_str(expression: &MalExpression, print_readably: bool) -> String {
    match expression {
        Int(i) => i.to_string(),
        Symbol(s) => s.to_string(),
        MalExpression::String(s) => pr_str_slice(s, print_readably),
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
        RustFunction(_) => "#<Rust function>".to_string(),
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
        Tco(x, _) => format!("#<TCO: ast = {}>", pr_str(x, print_readably)),
        Nil() => "nil".to_string(),
        Atom(a) => format!("(atom {})", pr_str(&a.borrow(), print_readably)),
    }
}

pub fn pr_str_slice(input_string: &str, print_readably: bool) -> String {
    if input_string.starts_with("\u{29e}") {
        format!(":{}", &input_string[2..])
    } else if print_readably {
        "\"".to_owned()
            + &input_string
                .replace("\\", "\\\\")
                .replace("\"", "\\\"")
                .replace("\n", "\\n")
            + "\""
    } else {
        input_string.to_string()
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
