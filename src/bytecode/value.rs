#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Null,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Null => write!(f, "null"),
        }
    }
}

// impl std::fmt::Display for Vec<Value> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "[ ");
//         for i in 0..self.len() {
//             write!(f, "{}, ", self.i);
//         }
//         write!(f, " ]")
//     }
// }

impl From<f64> for Value {
    fn from(val: f64) -> Self {
        Self::Number(val)
    }
}

impl From<i32> for Value {
    fn from(val: i32) -> Self {
        Self::Number(val.into())
    }
}

impl std::ops::Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        match self {
            Value::Number(n) => Value::Number(-n),
            _ => unimplemented!(),
        }
    }
}

// Arithmetic
impl std::ops::Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(x), Value::Number(y)) => Value::Number(x + y),
            _ => unimplemented!(),
        }
    }
}

impl std::ops::Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(x), Value::Number(y)) => Value::Number(x - y),
            _ => unimplemented!(),
        }
    }
}

impl std::ops::Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(x), Value::Number(y)) => Value::Number(x * y),
            _ => unimplemented!(),
        }
    }
}

impl std::ops::Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(x), Value::Number(y)) => Value::Number(x / y),
            _ => unimplemented!(),
        }
    }
}
