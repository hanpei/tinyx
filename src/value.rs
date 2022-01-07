use core::fmt;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Null,
    String(String),
    Boolean(bool),
    Int(i64),
    Float(f64),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(v) => write!(f, "{}", v),
            Value::Array(a) => fmt_array(a, f),
            Value::Object(o) => fmt_obj(o, f),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_display() {
        let array = vec![
            Value::String(String::from("abcd")),
            Value::Boolean(true),
            Value::Int(12),
            Value::Array(vec![Value::Int(1), Value::Int(2)]),
        ];

        let mut o = HashMap::new();
        o.insert("a".into(), Value::Int(1));

        let mut obj = HashMap::new();
        obj.insert("a".into(), Value::Int(1));
        obj.insert("b".into(), Value::String("xyz".to_string()));
        obj.insert("c".into(), Value::Null);
        obj.insert("d".into(), Value::Object(o));

        let v = vec![
            Value::Null,
            Value::String(String::from("abcd")),
            Value::Boolean(true),
            Value::Int(12),
            Value::Float(1.2),
            Value::Array(array),
            Value::Object(obj),
        ];

        for ele in v.iter() {
            println!("{}", ele);
        }
    }
}
