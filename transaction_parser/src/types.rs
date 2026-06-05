use std::fmt::{Display, Formatter};
use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Transaction {
    pub tx_id: u64,
    pub tx_type: TransactionType,
    pub from_user_id: u64,
    pub to_user_id: u64,
    pub amount: i64,
    pub timestamp: u64,
    pub status: TransactionStatus,
    pub description: String,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum TransactionType {
    #[default]
    Deposit,
    Transfer,
    Withdrawal,
    WrongType,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum TransactionStatus {
    #[default]
    Success,
    Failure,
    Pending,
    WrongStatus,
}

#[derive(Default)]
pub enum Format {
    Csv,
    #[default]
    Text,
    Binary,
}

impl TransactionStatus {
    pub(crate) fn as_bytes(&self) -> u8 {
        match self {
            Self::Success => 0,
            Self::Failure => 1,
            Self::Pending => 2,
            Self::WrongStatus => 3,
        }
    }
}

impl TransactionType {
    pub(crate) fn as_bytes(&self) -> u8 {
        match self {
            Self::Deposit => 0,
            Self::Transfer => 1,
            Self::Withdrawal => 2,
            Self::WrongType => 3,
        }
    }
}

impl Display for TransactionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Deposit => f.write_str("DEPOSIT"),
            Self::Transfer => f.write_str("TRANSFER"),
            Self::Withdrawal => f.write_str("WITHDRAWAL"),
            Self::WrongType => f.write_str("WRONG_TYPE"),
        }
    }
}

impl FromStr for TransactionType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DEPOSIT" => Ok(TransactionType::Deposit),
            "TRANSFER" => Ok(TransactionType::Transfer),
            "WITHDRAWAL" => Ok(TransactionType::Withdrawal),
            &_ => Err(Error::new(ErrorKind::InvalidInput, "TransactionType")),
        }
    }
}

impl Display for TransactionStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => f.write_str("SUCCESS"),
            Self::Failure => f.write_str("FAILURE"),
            Self::Pending => f.write_str("PENDING"),
            Self::WrongStatus => f.write_str("WRONG_STATUS"),
        }
    }
}

impl FromStr for TransactionStatus {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SUCCESS" => Ok(TransactionStatus::Success),
            "FAILURE" => Ok(TransactionStatus::Failure),
            "PENDING" => Ok(TransactionStatus::Pending),
            &_ => Err(Error::new(ErrorKind::InvalidInput, "неизвестный статус")),
        }
    }
}

impl From<u8> for TransactionType {
    fn from(value: u8) -> Self {
        match value {
            0 => TransactionType::Deposit,
            1 => TransactionType::Transfer,
            2 => TransactionType::Withdrawal,
            _ => TransactionType::WrongType,
        }
    }
}

impl From<u8> for TransactionStatus {
    fn from(value: u8) -> TransactionStatus {
        match value {
            0 => TransactionStatus::Success,
            1 => TransactionStatus::Failure,
            2 => TransactionStatus::Pending,
            _ => TransactionStatus::WrongStatus,
        }
    }
}
