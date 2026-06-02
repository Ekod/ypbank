use crate::common::{
    parse_str_to_i64, parse_str_to_transaction_status, parse_str_to_transaction_type,
    parse_str_to_u64,
};
use crate::errors::ParserError;
use crate::types::Transaction;
use std::io::{BufRead, Error, Read, Write};

const TXT_HEADER_PREFIX: char = '#';

pub fn read<R: Read>(reader: &mut R) -> Result<Vec<Transaction>, ParserError> {
    let mut transactions: Vec<Transaction> = Vec::new();
    let mut buf = Vec::<u8>::new();
    let _ = reader.read_to_end(&mut buf)?;
    let lines: Vec<Result<String, Error>> = buf.lines().collect();

    let transaction = &mut Transaction::default();
    for line in lines {
        let line = match line {
            Ok(line) => line,
            Err(err) => Err(ParserError::InvalidRecord(err.to_string()))?,
        };
        if line.starts_with(TXT_HEADER_PREFIX) {
            continue;
        }
        if line.is_empty() {
            transactions.push(transaction.clone());
            continue;
        }
        let transaction_fields = line.split(": ").collect::<Vec<&str>>();
        if transaction_fields.len() != 2 {
            return Err(ParserError::InvalidRecord(
                "неверная длина транзакции".to_string(),
            ));
        }

        let field_name = transaction_fields[0].trim();
        let field_value = transaction_fields[1].trim();
        match field_name {
            "TX_ID" => transaction.tx_id = parse_str_to_u64(field_value)?,
            "TX_TYPE" => transaction.tx_type = parse_str_to_transaction_type(field_value)?,
            "FROM_USER_ID" => transaction.from_user_id = parse_str_to_u64(field_value)?,
            "TO_USER_ID" => transaction.to_user_id = parse_str_to_u64(field_value)?,
            "AMOUNT" => transaction.amount = parse_str_to_i64(field_value)?,
            "TIMESTAMP" => transaction.timestamp = parse_str_to_u64(field_value)?,
            "STATUS" => transaction.status = parse_str_to_transaction_status(field_value)?,
            "DESCRIPTION" => transaction.description = field_value.to_string(),
            _ => {}
        }
    }

    Ok(transactions)
}

pub fn write<W: Write>(writer: &mut W, transactions: &Vec<Transaction>) -> Result<(), ParserError> {
    for transaction in transactions {
        writeln!(writer, "{}", TXT_HEADER_PREFIX)?;
        writeln!(writer, "TX_ID: {}", transaction.tx_id)?;
        writeln!(writer, "TX_TYPE: {}", transaction.tx_type)?;
        writeln!(writer, "FROM_USER_ID: {}", transaction.from_user_id)?;
        writeln!(writer, "TO_USER_ID: {}", transaction.to_user_id)?;
        writeln!(writer, "AMOUNT: {}", transaction.amount)?;
        writeln!(writer, "TIMESTAMP: {}", transaction.timestamp)?;
        writeln!(writer, "STATUS: {}", transaction.status)?;
        writeln!(writer, "DESCRIPTION: {}", transaction.description)?;
        writeln!(writer)?;
    }

    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_read_success() {}

    #[test]
    fn test_write_success() {}
}
