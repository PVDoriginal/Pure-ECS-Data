use std::{fmt::Display, ops};

#[derive(Clone, Debug, Default)]
pub enum Data {
    #[default]
    None,
    String(String),
    Num(Num),
    Bang,
}

#[derive(Clone, Debug)]
pub enum Num {
    Int(i32),
    Float(f32),
}

impl Default for Num {
    fn default() -> Num {
        Num::Int(0)
    }
}

impl Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Num::Float(n) => write!(f, "{n}"),
            Num::Int(n) => write!(f, "{n}"),
        }
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::None => write!(f, "Nothing."),
            Data::String(s) => write!(f, "{s}"),
            Data::Num(n) => write!(f, "{n}"),
            Data::Bang => write!(f, "Bang!"),
        }
    }
}

impl From<&str> for Data {
    fn from(value: &str) -> Self {
        Data::String(value.into())
    }
}

impl From<Num> for Data {
    fn from(value: Num) -> Self {
        Data::Num(value)
    }
}

impl From<Data> for Num {
    fn from(value: Data) -> Self {
        match value {
            Data::Num(n) => n,
            _ => Num::Int(0),
        }
    }
}

impl From<i32> for Data {
    fn from(value: i32) -> Self {
        Data::Num(Num::Int(value))
    }
}

impl From<f32> for Data {
    fn from(value: f32) -> Self {
        Data::Num(Num::Float(value))
    }
}

impl Data {
    fn string(&self) -> String {
        match self.clone() {
            Data::String(string) => string,
            Data::Num(n) => format!("{n}"),
            _ => "".into(),
        }
    }

    pub fn assign(&mut self, other: Data) {
        if matches!(self, Data::None) && matches!(other, Data::Bang) {
            *self = Data::Bang;
        } else if !matches!(other, Data::None | Data::Bang) {
            *self = other;
        }
    }
}

impl ops::Add<Num> for Num {
    type Output = Num;
    fn add(self, rhs: Num) -> Num {
        match (self, rhs) {
            (Num::Int(a), Num::Int(b)) => Num::Int(a + b),
            (Num::Float(a), Num::Float(b)) => Num::Float(a + b),
            (Num::Int(a), Num::Float(b)) => Num::Float(a as f32 + b),
            (Num::Float(a), Num::Int(b)) => Num::Float(a + b as f32),
        }
    }
}

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
            (Data::Num(a), Data::Num(b)) => Data::Num(a + b),
        }
    }
}

impl ops::AddAssign<Num> for Num {
    fn add_assign(&mut self, rhs: Num) {
        *self = self.clone() + rhs;
    }
}

impl ops::AddAssign<Data> for Data {
    fn add_assign(&mut self, rhs: Data) {
        *self = self.clone() + rhs;
    }
}
