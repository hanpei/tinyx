use std::collections::HashMap;

use crate::value::Value;

#[derive(Debug)]
pub struct ScopedSymbolTable {
    name: String,
    level: i32,
    symbols: HashMap<String, Value>,
}

impl ScopedSymbolTable {
    pub fn new(name: String, level: i32) -> Self {
        Self {
            symbols: HashMap::new(),
            name,
            level,
        }
    }

    pub fn insert(&mut self, k: String, v: Value) {
        println!("SymbolTable insert: {}", k);
        self.symbols.insert(k, v);
    }

    pub fn lookup(&mut self, k: String) -> Option<&Value> {
        println!("SymbolTable lookup: {}", k);
        self.symbols.get(&k)
    }
}
