use regex::Regex;
use std::iter::FromIterator;
use types::MalExpression;
#[derive(Debug)]
struct Reader<'a> {
    tokens: Vec<&'a str>,
    index: usize,
}

impl<'a> Reader<'a> {
    fn next(&mut self) -> Option<String> {
        // return current token, increment
        if self.tokens.len() > self.index {
            self.index += 1;
            let result = self.tokens[self.index - 1].to_string();
            //            println!("NEXT: {}", result);
            Some(result)
        } else {
            None
        }
    }
    fn peek(&self) -> Option<String> {
        // just peek at current token
        if self.tokens.len() > self.index {
            let result = self.tokens[self.index].to_string();
            //            println!("PEEK: {}", result);
            Some(result)
        } else {
            None
        }
    }
}

#[allow(deprecated)]
fn tokenize(line: &str) -> Vec<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r#"[\s,]*(?P<token>~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#
        )
        .unwrap();
    }
    let mut vec: Vec<&str> = Vec::new();
    for c in RE.captures_iter(line) {
        if let Some(x) = c.name("token") {
            vec.push(x.as_str())
        }
    }
    vec
}

pub(crate) fn read_str(line: &str) -> Result<MalExpression, String> {
    // call tokenize, create new Reader instance with tokens
    // call read_form with the Reader instance
    let tokenized = tokenize(line);
    let mut reader = Reader {
        tokens: tokenized,
        index: 0,
    };
    read_form(&mut reader)
}

fn read_form(reader: &mut Reader) -> Result<MalExpression, String> {
    //    println!("read_form: {}", format!("{:?}", reader));
    match reader.peek() {
        Some(token) => match token.as_ref() {
            "(" => read_list(reader),
            "[" => read_vector(reader),
            _ => read_atom(reader),
        },
        None => Err("unexpected EOF".to_string()),
    }
}

fn read_list(reader: &mut Reader) -> Result<MalExpression, String> {
    match read_sequence(reader, "(", ")") {
        Ok(sequence) => Ok(MalExpression::List(sequence)),
        Err(e) => Err(e),
    }
}

fn read_vector(reader: &mut Reader) -> Result<MalExpression, String> {
    match read_sequence(reader, "[", "]") {
        Ok(sequence) => Ok(MalExpression::Vector(sequence)),
        Err(e) => Err(e),
    }
}

fn read_sequence(
    reader: &mut Reader,
    opening_token: &str,
    closing_token: &str,
) -> Result<Vec<MalExpression>, String> {
    //    println!("read_sequence: {}", format!("{:?}", reader));
    let mut list_vec: Vec<MalExpression> = vec![];
    match reader.peek() {
        Some(token) => {
            if token == opening_token {
                reader.next(); // swallow opening token
            } else {
                return Err("internal error: opening token incorrect in read_sequence".to_string());
            }
        }
        None => {
            return Err(
                "internal error: EOF while reading opening token in read_sequence".to_string(),
            )
        }
    }
    loop {
        // TODO: more idiomatic way to do this?
        match reader.peek() {
            Some(token) => {
                if token == closing_token {
                    reader.next(); // swallow closing token
                    return Ok(list_vec);
                } else {
                    match read_form(reader) {
                        Ok(expression) => list_vec.push(expression),
                        Err(e) => return Err(e),
                    }
                }
            }
            None => return Err("EOF while reading sequence".to_string()),
        }
    }
}

fn read_atom(reader: &mut Reader) -> Result<MalExpression, String> {
    match reader.next() {
        Some(token) => match token.parse::<i32>() {
            Ok(number) => Ok(MalExpression::Int(number)),
            Err(_) => {
                let mut chars: Vec<char> = token.chars().collect();
                if !chars.is_empty() && chars[0] == '"' {
                    if chars.len() < 2 {
                        return Err(
                            "unbalanced string: ".to_string() + String::from_iter(chars).as_ref()
                        );
                    }
                    let mut result: Vec<char> = vec![];
                    for char in chars[1..chars.len() - 1].to_vec() {
                        // unescape?
                        result.push(char)
                    }
                    Ok(MalExpression::String(result.into_iter().collect()))
                } else {
                    Ok(MalExpression::Symbol(token))
                }
            }
        },
        None => Err("EOF while reading atom".to_string()),
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
            format!("{:?}", read_str(r#"(1 "a" 2 3 (c))"#).unwrap()),
            r#"List([Int(1), String("a"), Int(2), Int(3), List([Symbol("c")])])"#
        );
        assert_eq!(
            format!("{:?}", read_str("(")),
            r#"Err("EOF while reading sequence")"#
        );
    }
}
