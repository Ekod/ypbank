use crate::common::{
    parse_str_to_i64, parse_str_to_transaction_status, parse_str_to_transaction_type,
    parse_str_to_u64,
};
use crate::errors::ParserError;
use crate::types::Transaction;
use std::io::{BufRead, Error, Read, Write};

const CSV_HEADER: &str =
    "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION";

pub fn read<R: Read>(reader: &mut R) -> Result<Vec<Transaction>, ParserError> {
    let mut transactions: Vec<Transaction> = Vec::new();
    let mut buf = Vec::<u8>::new();
    let _ = reader.read_to_end(&mut buf)?;
    let mut lines: Vec<Result<String, Error>> = buf.lines().collect();
    match lines.first() {
        Some(header_line) => match header_line {
            Ok(header_line) => {
                let header = header_line.trim().to_string();
                if header != CSV_HEADER {
                    return Err(ParserError::InvalidHeader(
                        "csv header is invalid".to_string(),
                    ));
                }
            }
            Err(err) => {
                return Err(ParserError::InvalidHeader(err.to_string()));
            }
        },
        None => {
            return Err(ParserError::InvalidHeader("header not found".to_string()));
        }
    }

    let _ = lines.remove(0);

    for line in lines {
        let line = line.expect("Failed to read line");
        if line.is_empty() {
            continue;
        }

        let transaction_fields = line.split(",").collect::<Vec<&str>>();
        if transaction_fields.len() != 8 {
            return Err(ParserError::InvalidRecord(
                "transaction data length is incorrect".to_string(),
            ));
        }

        let tx_id = parse_str_to_u64(transaction_fields[0])?;
        let tx_type = parse_str_to_transaction_type(transaction_fields[1])?;
        let from_user_id = parse_str_to_u64(transaction_fields[2])?;
        let to_user_id = parse_str_to_u64(transaction_fields[3])?;
        let amount = parse_str_to_i64(transaction_fields[4])?;
        let timestamp = parse_str_to_u64(transaction_fields[5])?;
        let status = parse_str_to_transaction_status(transaction_fields[6])?;
        let description = transaction_fields[7].to_string();

        let transaction = Transaction {
            tx_id,
            tx_type,
            from_user_id,
            to_user_id,
            amount,
            timestamp,
            status,
            description,
        };

        transactions.push(transaction);
    }

    Ok(transactions)
}

pub fn write<W: Write>(writer: &mut W, transactions: &Vec<Transaction>) -> Result<(), ParserError> {
    writeln!(writer, "{CSV_HEADER}")?;

    for transaction in transactions {
        writeln!(
            writer,
            "{},{},{},{},{},{},{},{}",
            transaction.tx_id,
            transaction.tx_type,
            transaction.from_user_id,
            transaction.to_user_id,
            transaction.amount,
            transaction.timestamp,
            transaction.status,
            transaction.description,
        )?;
    }

    writer.flush()?;
    Ok(())
}
