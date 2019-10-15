use regex::Regex;
use types::MalExpression;

struct Reader<'a> {
    tokens: Vec<&'a str>,
    index: usize,
}

impl<'a> Reader<'a> {
    fn next(&mut self) -> Option<String> {
        // return current token, increment
        if self.tokens.len() > self.index {
            self.index += 1;
            Some(self.tokens[self.index - 1].to_string())
        } else {
            None
        }
    }
    fn peek(&self) -> Option<String> {
        // just peek at current token
        if self.tokens.len() > self.index {
            Some(self.tokens[self.index].to_string())
        } else {
            None
        }
    }
}

fn tokenize(line: &str) -> Vec<&str> {
    lazy_static! {
        static ref re: Regex = Regex::new(
            r#"[\s,]*(?P<token>~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#
        )
        .unwrap();
    }
    let mut vec: Vec<&str> = Vec::new();
    for c in re.captures_iter(line) {
        match c.name("token") {
            Some(x) => (vec.push(x.as_str())),
            None => (),
        }
    }
    vec
}

pub(crate) fn read_str(line: &str) -> Option<MalExpression> {
    // call tokenize, create new Reader instance with tokens
    // call read_form with the Reader instance
    let tokenized = tokenize(line);
    let mut reader = Reader {
        tokens: tokenized,
        index: 0,
    };
    read_form(&mut reader)
}

fn read_form(reader: &mut Reader) -> Option<MalExpression> {
    match reader.peek() {
        Some(token) => match token.as_ref() {
            "(" => read_list(reader),
            _ => read_atom(reader),
        },
        None => None,
    }
}

fn read_list(reader: &mut Reader) -> Option<MalExpression> {
    let mut list_vec: Vec<MalExpression> = vec![];
    match reader.next() {
        Some(token) => {
            if token != "(".to_string() {
                return None;
            }
        }
        None => return None,
    }
    loop {
        // TODO: more idiomatic way to do this?
        match reader.peek() {
            Some(token) => match token.as_ref() {
                ")" => return Some(MalExpression::List(list_vec)),
                _ => match read_form(reader) {
                    Some(expression) => list_vec.push(expression),
                    None => return None,
                },
            },
            None => return None,
        }
    }
}

fn read_atom(reader: &mut Reader) -> Option<MalExpression> {
    match reader.next() {
        Some(token) => match token.parse::<i32>() {
            Ok(number) => Some(MalExpression::Int(number)),
            Err(_) => {
                if token.chars().nth(0).unwrap() == '"' {
                    Some(MalExpression::String(token))
                } else {
                    Some(MalExpression::Symbol(token))
                }
            }
        },
        None => None,
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize("(1, 2 3)"), vec!["(", "1", "2", "3", ")"]);
    }

    #[test]
    fn test_read_str() {
        assert_eq!(
            format!("{:?}", read_str("(1 2 3)").unwrap()),
            "List([Int(1), Int(2), Int(3)])"
        );
        assert_eq!(
            format!("{:?}", read_str("(1 \"a\" 2 3 (c))").unwrap()),
            "List([Int(1), String(\"\\\"a\\\"\"), Int(2), Int(3), List([Symbol(\"c\")])])"
        );
        assert_eq!(format!("{:?}", read_str("(")), "None")
    }
}
