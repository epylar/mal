use regex::Regex;
use types::MalExpression;
use types::MalExpression::{List, Symbol, Vector, HashTable, Int};
use types::MalRet;
#[derive(Debug)]
struct Reader {
    tokens: Vec<String>,
    index: usize,
}

impl Reader {
    fn next(&mut self) -> Option<String> {
        // return current token, increment
        if self.tokens.len() > self.index {
            self.index += 1;
            let result = self.tokens[self.index - 1].clone();
            //            println!("NEXT: {}", result);
            Some(result)
        } else {
            None
        }
    }
    fn peek(&self) -> Option<String> {
        // just peek at current token
        if self.tokens.len() > self.index {
            let result = self.tokens[self.index].clone();
            //            println!("PEEK: {}", result);
            Some(result)
        } else {
            None
        }
    }
}

#[allow(deprecated)]
fn tokenize(line: &str) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r#"[\s,]*(?P<token>~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#
        )
        .unwrap();
    }
    let mut vec: Vec<String> = vec![];
    let re: &Regex = &RE; // helps IDE autocompletion
    for c in re.captures_iter(line) {
        if let Some(x) = c.name("token") {
            vec.push(x.as_str().to_string())
        }
    }
    vec
}

pub fn read_str(line: &str) -> MalRet {
    // call tokenize, create new Reader instance with tokens
    // call read_form with the Reader instance
    let tokenized = tokenize(line);
    let mut reader = Reader {
        tokens: tokenized,
        index: 0,
    };
    read_form(&mut reader)
}

fn read_form(reader: &mut Reader) -> MalRet {
    //    println!("read_form: {}", format!("{:?}", reader));
    let peek = reader.peek();
    match peek {
        Some(token) => match &token[..] {
            "(" => read_list(reader),
            "[" => read_vector(reader),
            "{" => read_hash_table(reader),
            "'" => read_quote(reader),
            "`" => read_quasiquote(reader),
            "~" => read_unquote(reader),
            "~@" => read_splice_unquote(reader),
            "@" => read_deref(reader),
            _ => read_atom(reader),
        },
        None => Err("unexpected EOF".to_string()),
    }
}

fn read_list(reader: &mut Reader) -> MalRet {
    match read_sequence(reader, "(", ")") {
        Ok(sequence) => Ok(List(sequence)),
        Err(e) => Err(e),
    }
}

fn read_vector(reader: &mut Reader) -> MalRet {
    match read_sequence(reader, "[", "]") {
        Ok(sequence) => Ok(Vector(sequence)),
        Err(e) => Err(e),
    }
}

fn read_hash_table(reader: &mut Reader) -> MalRet {
    match read_sequence(reader, "{", "}") {
        Ok(sequence) => Ok(HashTable(sequence)),
        Err(e) => Err(e),
    }
}

fn read_quote(reader: &mut Reader) -> MalRet {
    if reader.next() != Some("'".to_string()) {
        Err("internal error: expected '".to_string())
    } else {
        match read_form(reader) {
            Ok(x) => Ok(List(vec![
                Symbol("quote".to_string()),
                x,
            ])),
            Err(e) => Err(e),
        }
    }
}

fn read_quasiquote(reader: &mut Reader) -> MalRet {
    if reader.next() != Some("`".to_string()) {
        Err("internal error: expected `".to_string())
    } else {
        match read_form(reader) {
            Ok(x) => Ok(List(vec![
                Symbol("quasiquote".to_string()),
                x,
            ])),
            Err(e) => Err(e),
        }
    }
}

fn read_unquote(reader: &mut Reader) -> MalRet {
    if reader.next() != Some("~".to_string()) {
        Err("internal error: expected ~".to_string())
    } else {
        match read_form(reader) {
            Ok(x) => Ok(List(vec![
                Symbol("unquote".to_string()),
                x,
            ])),
            Err(e) => Err(e),
        }
    }
}

fn read_splice_unquote(reader: &mut Reader) -> MalRet {
    if reader.next() != Some("~@".to_string()) {
        Err("internal error: expected ~@".to_string())
    } else {
        match read_form(reader) {
            Ok(x) => Ok(List(vec![
                Symbol("splice-unquote".to_string()),
                x,
            ])),
            Err(e) => Err(e),
        }
    }
}

fn read_deref(reader: &mut Reader) -> MalRet {
    if reader.next() != Some("@".to_string()) {
        Err("internal error: expected @".to_string())
    } else {
        match read_form(reader) {
            Ok(x) => Ok(List(vec![
                Symbol("deref".to_string()),
                x,
            ])),
            Err(e) => Err(e),
        }
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

#[allow(deprecated)]
fn tokenize_quoted_string(string: &str) -> Vec<&str> {
    // '"abc"' -> ['"', 'a', 'b', 'c', '"']
    // '"a\"bc"' -> ['"', 'a', '\"', 'c', '"']
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"(?P<element>([^"\\])|(\\["n\\])|("))"#).unwrap();
    }
    let mut vec: Vec<&str> = Vec::new();
    for c in RE.captures_iter(string) {
        if let Some(x) = c.name("element") {
            vec.push(x.as_str())
        }
    }
    vec
}

fn escaped_to_char(string: &str) -> Result<char, String> {
    let chars: Vec<char> = string.chars().collect();
    if chars.len() == 1 {
        Ok(chars[0])
    } else if string == r#"\""# {
        Ok('"')
    } else if string == r#"\n"# {
        Ok('\n')
    } else if string == r#"\\"# {
        Ok('\\')
    } else {
        Err(format!("invalid escape sequence {}", string))
    }
}

fn unescape_string(string: &str) -> Result<String, String> {
    let tokens: Vec<&str> = tokenize_quoted_string(string);
    if tokens.len() < 2 || tokens.first().unwrap() != &"\"" || tokens.last().unwrap() != &"\"" {
        Err(format!("invalid or unbalanced string {}", string))
    } else {
        let result: Result<Vec<char>, String> = tokens
            .into_iter()
            .map(|element: &str| escaped_to_char(element))
            .collect();

        match result {
            Ok(r) => Ok(r[1..r.len() - 1].iter().collect()),
            Err(e) => Err(e),
        }
    }
}

fn read_atom(reader: &mut Reader) -> MalRet {
    match reader.next() {
        Some(token) => {
            if let Ok(number) = token.parse::<i32>() {
                return Ok(Int(number));
            }
            match token.chars().next() {
                None => Err("internal error: empty token".to_string()),
                Some('"') => match unescape_string(&token) {
                    Ok(s) => Ok(MalExpression::String(s)),
                    Err(e) => Err(e),
                },
                Some(':') => Ok(MalExpression::String(format!(
                    "\u{29e}{}",
                    token.chars().skip(1).collect::<String>()
                ))),
                Some(_) => Ok(Symbol(token.to_string())),
            }
        }
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
    fn test_tokenize_quoted_str() {
        assert_eq!(tokenize_quoted_string(r#""a""#), ["\"", "a", "\""]);
        assert_eq!(tokenize_quoted_string(r#""a\n"#), ["\"", "a", "\\n"])
    }

    #[test]
    fn test_unescape_str() {
        assert_eq!(unescape_string(r#""abc""#), Ok(r#"abc"#.to_string()));
        assert_eq!(
            unescape_string(r#""abc"#),
            Err(r#"invalid or unbalanced string "abc"#.to_string())
        );
        assert_eq!(
            format!("{:?}", unescape_string("abc")),
            r#"Err("invalid or unbalanced string abc")"#
        );
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
        assert_eq!(
            format!("{:?}", read_str("\"abc")),
            r#"Err("invalid or unbalanced string \"abc")"#
        );
    }
}
