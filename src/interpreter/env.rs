use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::value::Value;

pub type Env = Rc<RefCell<Environment>>;
pub trait EnvMethod {
    fn create() -> Rc<RefCell<Environment>>;
    fn extends(env: &Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>>;
    fn define(&mut self, name: String, value: Value);
    fn ancestor(&self, depth: usize) -> Self;
    fn get_at(&self, distance: usize, name: &str) -> Option<Value>;
    fn assign_at(&mut self, distance: usize, name: &str, value: Value) -> bool;
    fn get(&self, name: &str) -> Option<Value>;
    fn assign(&mut self, name: &str, value: Value) -> bool;
}

impl EnvMethod for Rc<RefCell<Environment>> {
    fn create() -> Self {
        Rc::new(RefCell::new(Environment::new()))
    }

    fn extends(env: &Rc<RefCell<Environment>>) -> Self {
        let env = Environment {
            store: HashMap::new(),
            outer: Some(Rc::clone(env)),
        };
        Rc::new(RefCell::new(env))
    }

    fn define(&mut self, name: String, value: Value) {
        self.borrow_mut().store.insert(name, value);
    }

    fn ancestor(&self, distance: usize) -> Self {
        let mut environment = self.clone();

        for i in 0..distance {
            let parent = self
                .borrow()
                .outer
                .clone()
                .unwrap_or_else(|| panic!("No enclosing environment at {}", i));
            environment = Rc::clone(&parent);
        }
        environment
    }

    fn get_at(&self, distance: usize, name: &str) -> Option<Value> {
        let key = name;
        if distance > 0 {
            self.ancestor(distance).borrow().get(key)
        } else {
            self.get(key)
        }
    }

    fn assign_at(&mut self, distance: usize, name: &str, value: Value) -> bool {
        if distance > 0 {
            self.ancestor(distance).borrow_mut().assign(name, value)
        } else {
            self.assign(name, value)
        }
    }

    fn get(&self, name: &str) -> Option<Value> {
        self.borrow().get(name)
    }

    fn assign(&mut self, name: &str, value: Value) -> bool {
        self.borrow_mut().assign(name, value)
    }
}

#[derive(Clone, Debug)]
pub struct Environment {
    store: HashMap<String, Value>,
    outer: Option<Env>,
}

impl Environment {
    fn new() -> Self {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    fn get(&self, name: &str) -> Option<Value> {
        let key = name;
        match self.store.get(key) {
            Some(value) => Some((*value).clone()),
            None => match self.outer {
                Some(ref outer) => outer.borrow().get(name),
                None => None,
            },
        }
    }

    fn assign(&mut self, name: &str, value: Value) -> bool {
        if self.store.contains_key(name) {
            self.store.insert(name.to_string(), value);
            true
        } else {
            match self.outer {
                Some(ref outer) => outer.borrow_mut().assign(name, value),
                None => false,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_env() -> Rc<RefCell<Environment>> {
        let mut first = Env::create();
        first.define("name".to_string(), Value::String("abc".to_string()));

        let mut second = Env::extends(&first);
        second.define("age".to_string(), Value::Number(1.0));

        second
    }

    #[test]
    fn env_lookup() {
        let env = get_env();
        let name = env.get("name").unwrap();
        let age = env.get("age").unwrap();

        assert_eq!(name, Value::String("abc".to_string()));
        assert_eq!(age, Value::Number(1.0));
    }

    #[test]
    fn env_assign() {
        let mut env = get_env();

        env.assign("name", Value::String("xyz".to_string()));
        env.assign("age", Value::Number(2.0));

        println!("env: {:#?}", env);

        let name = env.get("name").unwrap();
        let age = env.get("age").unwrap();

        assert_eq!(name, Value::String("xyz".to_string()));
        assert_eq!(age, Value::Number(2.0));
    }
}
