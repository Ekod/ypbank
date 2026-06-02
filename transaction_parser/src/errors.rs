use std::fmt::{Display, Formatter};
use std::io;
use std::string::FromUtf8Error;

#[derive(Debug, PartialEq)]
pub enum ParserError {
    IO(String),
    InvalidArgument(String),
    InvalidHeader(String),
    InvalidRecord(String),
    InvalidValue(String),
    InvalidRange(String),
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
            Self::InvalidRange(err) => {
                write!(f, "некорректный диапазон {err}")
            }
        }
    }
}

impl From<io::Error> for ParserError {
    fn from(err: io::Error) -> Self {
        Self::IO(err.to_string())
    }
}

impl From<FromUtf8Error> for ParserError {
    fn from(err: FromUtf8Error) -> Self {
        Self::InvalidArgument(err.to_string())
    }
}
