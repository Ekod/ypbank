use std::fmt::{Display, Formatter};
use std::io;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum ParserError {
    IO(io::Error),
    InvalidArgument(String),
    InvalidHeader(String),
    InvalidRecord(String),
    InvalidValue(String),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(err) => write!(f, "ошибка ввода-вывода {err}"),
            Self::InvalidArgument(err) => write!(f, "некорректный аргумент {err}"),
            Self::InvalidHeader(err) => {
                write!(f, "некорректный заголовок {err}")
            }
            Self::InvalidRecord(err) => {
                write!(f, "некорректная запись {err}")
            }
            Self::InvalidValue(err) => {
                write!(f, "некорректное значение {err}")
            }
        }
    }
}

impl From<io::Error> for ParserError {
    fn from(err: io::Error) -> Self {
        Self::IO(err)
    }
}

impl From<FromUtf8Error> for ParserError {
    fn from(err: FromUtf8Error) -> Self {
        Self::InvalidArgument(err.to_string())
    }
}
