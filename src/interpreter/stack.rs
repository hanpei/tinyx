use std::collections::HashMap;

use crate::value::Value;

type Item = HashMap<String, Value>;

#[derive(Debug, PartialEq, Clone)]
pub enum ARKind {
    Program,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ActiveRecord {
    name: String,
    kind: ARKind,
    level: i32,
    store: Item,
}

impl ActiveRecord {
    pub fn new(name: &str, kind: ARKind, level: i32) -> Self {
        Self {
            name: name.into(),
            kind,
            level,
            store: HashMap::new(),
        }
    }

    pub fn default() -> Self {
        Self {
            name: "main".to_string(),
            kind: ARKind::Program,
            level: 1,
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: Value) {
        self.store.insert(key.into(), value);
    }

    pub fn get(&mut self, key: &str) -> Option<Value> {
        match self.store.get(key) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CallStack {
    items: Vec<ActiveRecord>,
}

impl CallStack {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push(&mut self, item: ActiveRecord) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<ActiveRecord> {
        self.items.pop()
    }

    pub fn peek(&mut self) -> Option<ActiveRecord> {
        let size = self.items.len();
        let item = self.items.get(size - 1);
        match item {
            Some(i) => Some(i.clone()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_stack_method() {
        let mut ar = ActiveRecord::new("main", ARKind::Program, 1);
        ar.set("name", Value::String("tom".to_string()));
        println!("ar = {:?}", ar);

        let mut stack = CallStack::new();
        stack.push(ar);

        let map = HashMap::from([("name".to_string(), Value::String("tom".to_string()))]);
        let espect = CallStack {
            items: vec![ActiveRecord {
                store: map,
                name: "main".to_string(),
                kind: ARKind::Program,
                level: 1,
            }],
        };

        println!("stack = {:#?}", stack);

        assert_eq!(stack, espect);

        let peeked = stack.peek().unwrap();
        let mut espect = ActiveRecord::default();
        espect.set("name", Value::String("tom".to_string()));
        assert_eq!(peeked, espect);

        let last = stack.pop().unwrap();
        assert_eq!(last, espect);
    }
}
