use crate::types::MalExpression;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Env {
    outer: Option<Box<Env>>,
    data: HashMap<String, MalExpression>,
}

impl Env {
    pub fn new(outer: Option<Box<Env>>, data: HashMap<String, MalExpression>) -> Env {
        Env { outer, data }
    }

    pub fn set(&mut self, key: &str, val: MalExpression) {
        self.data.insert(key.to_string(), val);
    }

    pub fn get(&self, key: &str) -> Option<&MalExpression> {
        match self.data.get(key) {
            Some(result) => Some(result),
            None => match &self.outer {
                Some(env) => env.get(key),
                None => None,
            },
        }
    }
}
