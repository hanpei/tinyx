use core::fmt;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{ast::Statement, interpreter::Environment};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Null,
    String(String),
    Boolean(bool),
    Number(f64),
    Function(Function),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b == true,
            Value::Null => false,
            // Value::String(_) => todo!(),
            // Value::Number(_) => todo!(),
            // Value::Array(_) => todo!(),
            // Value::Object(_) => todo!(),
            _ => true,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Number(v) => write!(f, "{}", v),
            Value::Array(a) => fmt_array(a, f),
            Value::Object(o) => fmt_obj(o, f),
            Value::Function(fun) => fmt_fn(fun, f),
        }
    }
}

fn fmt_array(array: &Vec<Value>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
        f,
        "[ {} ]",
        array
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    )
}

fn fmt_obj(obj: &HashMap<String, Value>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
        f,
        "{{ {} }}",
        obj.iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<String>>()
            .join(", ")
    )
}

fn fmt_fn(fun: &Function, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
        f,
        "[Function <{}>]",
        match &fun.name {
            Some(n) => n,
            None => "anonymous",
        }
    )
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub name: Option<String>,
    pub params: Vec<String>,
    pub body: Box<Statement>,
    pub scope: Rc<RefCell<Environment>>,
}

impl Function {
    pub fn new(
        name: Option<String>,
        params: Vec<String>,
        body: Statement,
        scope: Rc<RefCell<Environment>>,
    ) -> Self {
        Function {
            name,
            params,
            body: Box::new(body),
            scope,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_display() {
        let array = vec![
            Value::String(String::from("abcd")),
            Value::Boolean(true),
            Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]),
        ];

        let mut o = HashMap::new();
        o.insert("a".into(), Value::Number(1.0));

        let mut obj = HashMap::new();
        obj.insert("a".into(), Value::Number(1.0));
        obj.insert("b".into(), Value::String("xyz".to_string()));
        obj.insert("c".into(), Value::Null);
        obj.insert("d".into(), Value::Object(o));

        let v = vec![
            Value::Null,
            Value::String(String::from("abcd")),
            Value::Boolean(true),
            Value::Number(1.2),
            Value::Array(array),
            Value::Object(obj),
        ];

        for ele in v.iter() {
            println!("{}", ele);
        }
    }
}
