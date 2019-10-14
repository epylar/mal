use regex::Regex;

trait Reader {

}

struct ReaderStruct();

impl Reader for ReaderStruct {

}

fn tokenize(line: &str) -> Vec<&str> {
    let mut vec = Vec::new();
    vec.push(line);
    let re = Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#).unwrap();
    vec
}