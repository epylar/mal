use std::rc::Rc;

#[derive (Clone)]
pub enum MalType {
    True,
    False,
    Nil,
    MString(String),
    Symbol(String),
    List(Vec<MalVal>),
    Vector(Vec<MalVal>),
    Integer(i32),
    Quote(MalVal),
    Quasiquote(MalVal),
    Unquote(MalVal),
    Spliceunquote(MalVal),
    Func(fn(Vec<MalVal>) -> MalRet),
}

pub type MalVal = Rc<MalType>;
pub type MalRet = Result<MalVal, String>;


impl MalType {
    pub fn pr_str(&self) -> String {
        // This file will contain a single function pr_str which does the opposite of read_str:
        // take a mal data structure and return a string representation of it. But pr_str is much
        // simpler and is basically just a switch statement on the type of the input object.
        match *self {
            MalType::Nil => "nil".to_owned(),
            MalType::True => "true".to_owned(),
            MalType::False => "false".to_owned(),
            MalType::MString(ref s) => format!("\"{}\"", s),
            MalType::List(ref l) => self.pr_delimited(&l, "(", ")"),
            MalType::Vector(ref a) => self.pr_delimited(&a, "[", "]"),
            MalType::Integer(ref x) => x.to_string(),
            MalType::Symbol(ref s) => s.to_owned(),
            MalType::Quote(ref s) => format!("(quote {})", s.pr_str()),
            MalType::Quasiquote(ref s) => format!("(quasiquote {})", s.pr_str()),
            MalType::Unquote(ref s) => format!("(unquote {})", s.pr_str()),
            MalType::Spliceunquote(ref s) => format!("(splice-unquote {})", s.pr_str()),
            MalType::Func(_) => "<function ...>".to_owned()
        }
    }

    pub fn apply(&self, args: Vec<MalVal>) -> MalRet {
        //println!("apply to {}", self.pr_str());
        match *self {
            MalType::Func(ref f) => {
                f(args)
            }
            _ => Err(format!("tried to apply {} which is not a function", self.pr_str())),
        }
    }

    fn pr_delimited(&self, v: &[MalVal], left_delim: &str, right_delim: &str) -> String {
        let mut result: String = String::new();
        result.push_str(&left_delim);
        let mut first: bool = true;
        for s in v {
            if !first {
                result.push_str(" ");
            }
            result.push_str(&s.pr_str());
            first = false;
        }
        result.push_str(&right_delim);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::MalType::*;
    use std::rc::Rc;

    #[test]
    fn pr_str_works() {
        assert_eq!("nil", Nil.pr_str());
        assert_eq!("true", True.pr_str());
        assert_eq!("false", False.pr_str());
        assert_eq!("(nil true)",
                   List(vec![Rc::new(Nil), Rc::new(True)]).pr_str());
        let result = List(vec![Rc::new(Nil), Rc::new(Integer(42))]).pr_str();
        assert_eq!("(nil 42)", result)
    }
}
