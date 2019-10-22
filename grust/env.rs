use std::collections::HashMap;
use crate::types::MalExpression;


#[derive(Debug)]
pub struct Env {
    outer: Option<Box<Env>>,
    data: HashMap<String, MalExpression>
}


impl Env {
    pub fn new(outer: Option<Box<Env>>, data: HashMap<String, MalExpression>) -> Env {
        Env { outer: outer, data: data }
    }

    pub fn set(&mut self, key: String, val: MalExpression) {
        self.data.insert(key, val);
    }

    pub fn get(&self, key: &String) -> Option<&MalExpression> {
        self.data.get(key)
    }
}