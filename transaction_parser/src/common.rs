use crate::errors::ParserError;
use crate::types::{TransactionStatus, TransactionType};

pub fn parse_str_to_u64(field: &str) -> Result<u64, ParserError> {
    match field.parse::<u64>() {
        Ok(value) => Ok(value),
        Err(err) => Err(ParserError::InvalidValue(err.to_string())),
    }
}

pub fn parse_str_to_i64(field: &str) -> Result<i64, ParserError> {
    match field.parse::<i64>() {
        Ok(value) => Ok(value),
        Err(err) => Err(ParserError::InvalidValue(err.to_string())),
    }
}

pub fn parse_str_to_transaction_type(value: &str) -> Result<TransactionType, ParserError> {
    match value.parse::<TransactionType>() {
        Ok(value) => Ok(value),
        Err(err) => Err(ParserError::InvalidValue(err.to_string())),
    }
}

pub fn parse_str_to_transaction_status(value: &str) -> Result<TransactionStatus, ParserError> {
    match value.parse::<TransactionStatus>() {
        Ok(value) => Ok(value),
        Err(err) => Err(ParserError::InvalidValue(err.to_string())),
    }
}
