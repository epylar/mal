use regex::{Regex};

struct Reader<'a> {
    tokens: Vec<&'a str>
}

impl Reader<'_> {
    fn new(tokens: Vec<&str>) -> Reader {
        Reader { tokens: tokens.clone() }
    }
    fn next(&mut self) -> &str {
        // return current token, increment
        self.tokens[0]
    }
    fn peek(&self) -> &str {
        // just peek at current token
        "abc"
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

fn read_str(line: &str) -> &str {
    // call tokenize, create new Reader instance with tokens
    // call read_form with the Reader instance
    read_form(Reader::new(tokenize(line)))
}

fn read_form(reader: &mut Reader) -> &'static str {
    // This function will peek at the first token in the Reader object and switch on the first
    // character of that token. If the character is a left paren then read_list is called with
    // the Reader object. Otherwise, read_atom is called with the Reader Object. The return value
    // from read_form is a mal data type. If your target language is statically typed then you will
    // need some way for read_form to return a variant or subclass type. For example, if your
    // language is object oriented, then you can define a top level MalType (in types.qx) that
    // all your mal data types inherit from. The MalList type (which also inherits from MalType)
    // will contain a list/array of other MalTypes. If your language is dynamically typed then
    // you can likely just return a plain list/array of other mal types.
    "abc"
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize("(1, 2 3)"), vec!["(", "1", "2", "3", ")"]);
    }
}
