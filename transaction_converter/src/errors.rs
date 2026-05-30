use std::fmt::{Display, Formatter};

pub enum ConverterError {
    UnknownFormat(String),
}

impl Display for ConverterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownFormat(err) => write!(f, "неизвестный формат {err}"),
        }
    }
}
