use std::fmt::{Display, Formatter};

pub enum ComparerError {
    UnknownFormat(String),
}

impl Display for ComparerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownFormat(err) => write!(f, "неизвестный формат {err}"),
        }
    }
}
