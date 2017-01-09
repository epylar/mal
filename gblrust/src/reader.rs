use regex::Regex;
use types::MalType::*;
use types::MalType;
use types::MalVal;
use std::rc::Rc;
 
pub fn tokenizer(input: &str) -> Vec<String> {    
    let pattern = r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"|;.*|[^\s\[\]{}('"`,;)]+)"###;

    // it is okay if unwrap fails. we want to exit at this point.
    let mut results: Vec<String> = vec![];
    for result in Regex::new(pattern).unwrap().captures_iter(input) {
        results.push(result.at(1).unwrap().to_owned())
    }
    results
}

pub struct MalReader<>  {
    tokens: Vec<String>,
    pos: usize
}
impl Iterator for MalReader {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        if self.pos < self.tokens.len() {
            self.pos = self.pos+1;
            Some(self.tokens[self.pos-1].to_owned())
        } else {
            None
        }
    }
}
impl MalReader {
    fn peek(&self) -> Option<String> {
        if self.pos < self.tokens.len() {
           Some(self.tokens[self.pos].to_owned())
        } else {
           None
        }
    }
    fn read_form(&mut self) -> Result<MalType, String> {
        match self.peek() {
            Some(x) => match x {
                ref s if *s == "(" => self.read_list(),
                ref s if *s == "[" => self.read_vector(),
                ref s if *s ==  "'" => {
                    self.next();
                    self.read_form().map(|i| Quote(Rc::new(i)))
                },
                ref s if *s == "`" => {
                    self.next();
                    self.read_form().map(|i| Quasiquote(Rc::new(i)))
                },
                ref s if *s == "~" => {
                    self.next();
                    self.read_form().map(|i| Unquote(Rc::new(i)))
                },
                ref s if *s == "~@" => {
                    self.next();
                    self.read_form().map(|i| Spliceunquote(Rc::new(i)))
                },
                _ => read_atom(&(self.next().unwrap()))
            },
            None => Err("expected form, got EOF".to_owned()),
        }
    }

    fn read_delim(&mut self, left_token: &str, right_token: &str) -> Result<Vec<MalVal>, String> {
        assert_eq!(left_token, self.next().unwrap());
        let mut output: Vec<MalVal> = vec![];
        while self.peek() != Some(right_token.to_owned()) {
            match self.peek() {
                Some(_) => match self.read_form() {
                    Ok(y) => output.push(Rc::new(y)),
                    Err(z) => return Err(z)
                },
                None => return Err(format!("expected '{}', got EOF", right_token).to_owned())
            }
        }
        self.next();
        Ok(output)
    }

    fn read_list(&mut self) -> Result<MalType, String> {
        match self.read_delim("(",")") {
            Ok(x) => Ok(List(x)),
            Err(y) => Err(y)
        }
    }

    fn read_vector(&mut self) -> Result<MalType, String> {
        match self.read_delim("[", "]") {
            Ok(x) => Ok(Vector(x)),
            Err(y) => Err(y)
        }
    }
}


pub fn read_str(a: &str) -> Result<MalType, String> {
    // call tokenizer and create new Reader instance with tokens (with tokenizer)
    // then call read_form with reader instance
    let mut r = MalReader { tokens: tokenizer(a), pos: 0 };
    let form = r.read_form();
    match r.peek() {
        Some(x) => Err(format!("Did not expect any more tokens at end of form while reading input but saw {}", x).to_owned()),
        None => form
    }
}



// This function will look at the contents of the token and return the appropriate scalar
//(simple/single) data type value. Initially, you can just implement numbers (integers)
//and symbols . This will allow you to proceed through the next couple of steps before
//you will need to implement the other fundamental mal types: nil, true, false, and
//string. The remaining mal types: keyword, vector, hash-map, and atom do not need to
//be implemented until step 9 (but can be implemented at any point between this step and
//that). BTW, symbols types are just an object that contains a single string name value
//(some languages have symbol types already).

pub fn read_atom (token: &str) -> Result<MalType, String> {
    match token.chars().next() {
        Some(x) =>
            if x == '"' {
                Ok(MalType::MString(token[1..(token.len()-1)].to_owned()))
            } else {
                match token {
                    "nil" => Ok(Nil),
                    "true" => Ok(True),
                    "false" => Ok(False),
                    _ => {
                        let try_parse = token.parse::<i32>();
                        match try_parse {
                            Ok(x) => Ok(Integer(x)),
                            _ => Ok(Symbol(token.to_owned()))
                        }
                    }
                }
            },
        None => Err("empty token".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use reader;
    use types::MalType::*;
    use reader::read_atom;

    #[test]
    fn tokenizer_works() {
        let result = reader::tokenizer("(1 2)");
        assert_eq!(vec!["(", "1", "2", ")"], result)
    }

    #[test]
    fn read_str_works() {
        let x = reader::read_str("1").unwrap();
        match x {
            Integer(1) => (),
            _ => assert!(false)
        }
        let outstr = reader::read_str("(+ 1 2)").unwrap().pr_str();
        assert_eq!("(+ 1 2)", outstr);
    }



    #[test]
    fn read_atom_works() {
        let mut atom = read_atom("\"abc\"").unwrap();
        match atom {
            MString(x) => assert_eq!(x, "abc"),
            _ => assert!(false)
        }
        atom = read_atom("true").unwrap();
        match atom {
            True => (),
            _ => assert!(false)
        }
        atom = read_atom("1").unwrap();
        match atom {
            Integer(1) => (),
            _ => assert!(false)
        }
        atom = read_atom("42").unwrap();
        match atom {
            Integer(42) => (),
            _ => assert!(false)
        }
        let result = read_atom(".3");
        match result {
            Ok(Symbol(x)) => assert_eq!(".3", x),
            _ => assert!(false)
        }
    }
}
