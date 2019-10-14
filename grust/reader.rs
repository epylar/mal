use regex::{Captures, Regex};

trait Reader {}

struct ReaderStruct();

impl Reader for ReaderStruct {}

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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize("(1, 2 3)"), vec!["(", "1", "2", "3", ")"]);
    }
}
