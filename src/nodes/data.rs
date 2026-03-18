use std::fmt::Display;

#[derive(Clone, Debug, Default)]
pub enum Data {
    #[default]
    None,
    String(String),
    Bang,
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::None => write!(f, "Nothing."),
            Data::String(s) => write!(f, "{s}"),
            Data::Bang => write!(f, "Bang!"),
        }
    }
}

impl From<&str> for Data {
    fn from(value: &str) -> Self {
        Data::String(value.into())
    }
}
