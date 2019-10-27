use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::types::MalExpression;

#[derive(Debug, Clone)]
pub struct Env {
    outer: Option<Rc<Env>>,
    data: Rc<RefCell<HashMap<String, MalExpression>>>,
}

impl Env {
    pub fn simple_new(outer: Option<Env>) -> Result<Env, String> {
        Env::new(outer, Rc::new(vec![]), Rc::new(vec![]))
    }

    pub fn new(
        outer: Option<Env>,
        binds: Rc<Vec<String>>,
        exprs: Rc<Vec<MalExpression>>,
    ) -> Result<Env, String> {
        let mut data: HashMap<String, MalExpression> = HashMap::new();
        if !binds.is_empty() {
            for i in 0..binds.len() {
                if binds[i] == "&" {
                    if binds.len() > (i + 1) {
                        data.insert(
                            binds[i + 1].clone(),
                            MalExpression::List(Rc::new(exprs[i..].to_vec())),
                        );
                    } else {
                        return Err("no elements in binds after &".to_string());
                    }
                    break;
                }
                data.insert(binds[i].clone(), exprs[i].clone());
            }
        }
        Ok(Env {
            outer: match outer {
                Some(e) => Some(Rc::new(e)),
                None => None,
            },
            data: Rc::new(RefCell::new(data)),
        })
    }

    pub fn set(&mut self, key: &str, val: MalExpression) {
        self.data.borrow_mut().insert(key.to_string(), val);
    }

    pub fn get(&self, key: &str) -> Option<MalExpression> {
        match self.data.borrow().get(key) {
            Some(result) => Some(result.clone()),
            None => match &self.outer {
                Some(env) => env.get(key),
                None => None,
            },
        }
    }
}
