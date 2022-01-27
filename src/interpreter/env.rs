use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::value::Value;

#[derive(PartialEq, Clone, Debug)]
pub struct Environment {
    store: HashMap<String, Value>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn default() -> Rc<RefCell<Environment>> {
        Rc::new(RefCell::new(Environment::new()))
    }

    pub fn extends(env: &Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
        let env = Environment {
            store: HashMap::new(),
            outer: Some(Rc::clone(env)),
        };
        Rc::new(RefCell::new(env))
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.store.insert(name, value);
    }

    fn ancestor(&self, distance: usize) -> Rc<RefCell<Environment>> {
        let mut environment = Rc::new(RefCell::new(self.clone()));

        for i in 0..distance {
            let parent = self
                .outer
                .clone()
                .expect(&format!("No enclosing environment at {}", i));
            environment = Rc::clone(&parent);
        }
        environment
    }

    pub fn get_at(&self, distance: usize, name: &str) -> Option<Value> {
        let key = name;
        if distance > 0 {
            self.ancestor(distance).borrow().get(key)
        } else {
            self.get(key)
        }
    }

    pub fn assign_at(&mut self, distance: usize, name: &str, value: Value) -> bool {
        if distance > 0 {
            self.ancestor(distance).borrow_mut().assign(name, value)
        } else {
            self.assign(name, value)
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        let key = name;
        match self.store.get(key) {
            Some(value) => Some((*value).clone()),
            None => match self.outer {
                Some(ref outer) => outer.borrow().get(name),
                None => None,
            },
        }
    }

    pub fn assign(&mut self, name: &str, value: Value) -> bool {
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn get_env() -> Rc<RefCell<Environment>> {
//         let first = Environment::default();
//         first
//             .borrow_mut()
//             .define("name".to_string(), Value::String("abc".to_string()));

//         let second = Environment::extends(&first);
//         second
//             .borrow_mut()
//             .define("age".to_string(), Value::Number(1.0));

//         second
//     }

//     #[test]
//     fn env_lookup() {
//         let env = get_env();
//         let name = env.borrow_mut().lookup("name").unwrap();
//         let age = env.borrow_mut().lookup("age").unwrap();

//         assert_eq!(name, Value::String("abc".to_string()));
//         assert_eq!(age, Value::Number(1.0));
//     }

//     #[test]
//     fn env_assign() {
//         let env = get_env();

//         env.borrow_mut()
//             .assign("name", Value::String("xyz".to_string()));
//         env.borrow_mut().assign("age", Value::Number(2.0));

//         println!("env: {:#?}", env);

//         let name = env.borrow_mut().lookup("name").unwrap();
//         let age = env.borrow_mut().lookup("age").unwrap();

//         assert_eq!(name, Value::String("xyz".to_string()));
//         assert_eq!(age, Value::Number(2.0));
//     }
// }
