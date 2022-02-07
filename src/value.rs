use core::fmt;
use std::collections::HashMap;

use crate::interpreter::{
    class::{Class, Instance},
    function::Function,
};

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    String(String),
    Boolean(bool),
    Number(f64),
    Function(Function),
    Class(Class),
    Instance(Instance),
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

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::Function(l0), Self::Function(r0)) => l0.name == r0.name,
            (Self::Class(l0), Self::Class(r0)) => l0.id == r0.id,
            (Self::Instance(l0), Self::Instance(r0)) => l0.class.id == r0.class.id,
            (Self::Array(l0), Self::Array(r0)) => l0 == r0,
            (Self::Object(l0), Self::Object(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
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
            Value::Class(c) => fmt_class(c, f),
            Value::Instance(i) => fmt_instance(i, f),
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
    let fn_name = match &fun.name {
        Some(n) => n.to_string(),
        None => "anonymous".to_string(),
    };
    write!(f, "<fn {}>", fn_name)
}

fn fmt_class(class: &Class, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "<class {}>", class.id.name)
}

fn fmt_instance(instance: &Instance, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let name = instance.class_name();
    write!(f, "<instance of {}>", name)
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
