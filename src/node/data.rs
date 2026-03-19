use std::{fmt::Display, ops};

#[derive(Clone, Debug, Default)]
pub enum Data {
    #[default]
    None,
    String(String),
    Int(i32),
    Float(f32),
    Bang,
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::None => write!(f, "Nothing."),
            Data::String(s) => write!(f, "{s}"),
            Data::Int(n) => write!(f, "{n}"),
            Data::Float(n) => write!(f, "{n}"),
            Data::Bang => write!(f, "Bang!"),
        }
    }
}

impl From<&str> for Data {
    fn from(value: &str) -> Self {
        Data::String(value.into())
    }
}

impl Data {
    fn string(&self) -> String {
        match self.clone() {
            Data::String(string) => string,
            Data::Int(n) => format!("{n}"),
            Data::Float(n) => format!("{n}"),
            _ => "".into(),
        }
    }
}

#[derive(Default)]
pub struct DynData(Vec<Data>);

impl ops::Add<Data> for Data {
    type Output = Data;

    fn add(self, other: Data) -> Data {
        match (self, other) {
            (Data::None, d) => d,
            (d, Data::None) => d,
            (Data::String(s), x) => {
                let mut string = s;
                string.push_str(&x.string());

                Data::String(string)
            }
            (x, Data::String(s)) => {
                let mut string = s;
                string.push_str(&x.string());

                Data::String(string)
            }
            (Data::Bang, d) => d,
            (d, Data::Bang) => d,
            (Data::Int(a), Data::Int(b)) => Data::Int(a + b),
            (Data::Float(a), Data::Float(b)) => Data::Float(a + b),
            (Data::Int(a), Data::Float(b)) => Data::Float(a as f32 + b),
            (Data::Float(a), Data::Int(b)) => Data::Float(a + b as f32),
        }
    }
}

impl ops::AddAssign<Data> for Data {
    fn add_assign(&mut self, rhs: Data) {
        *self = self.clone() + rhs;
    }
}
