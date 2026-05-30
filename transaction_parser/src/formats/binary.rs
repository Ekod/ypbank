use crate::errors::ParserError;
use crate::types::{Transaction, TransactionStatus, TransactionType};
use std::io::{Read, Write};

const BIN_HEADER_ID: [u8; 4] = *b"YPBN";

pub fn read<R: Read>(reader: &mut R) -> Result<Vec<Transaction>, ParserError> {
    let mut transactions: Vec<Transaction> = Vec::new();

    loop {
        let mut header = [0u8; 4];
        let _ = reader.read_exact(&mut header);
        if !header.eq(BIN_HEADER_ID.as_slice()) {
            break;
        }
        let mut size = [0u8; 4];
        let _ = reader.read_exact(&mut size);
        let body_size = u32::from_be_bytes(size);
        let mut body = vec![0u8; body_size as usize];
        let _ = reader.read_exact(&mut body);

        let mut cursor = 0;

        let tx_id = parse_raw_bytes_to_u64_be(&body, &mut cursor);
        let tx_type = TransactionType::from(parse_raw_bytes_to_u8_be(&body, &mut cursor));
        let from_user_id = parse_raw_bytes_to_u64_be(&body, &mut cursor);
        let to_user_id = parse_raw_bytes_to_u64_be(&body, &mut cursor);
        let amount = parse_raw_bytes_to_i64_be(&body, &mut cursor);
        let timestamp = parse_raw_bytes_to_u64_be(&body, &mut cursor);
        let status = TransactionStatus::from(parse_raw_bytes_to_u8_be(&body, &mut cursor));
        let description_length = parse_raw_bytes_to_u32_be(&body, &mut cursor);
        let description = if description_length == 0 {
            String::new()
        } else {
            let mut buf = vec![0u8; description_length as usize];
            let end = cursor + description_length as usize;
            let bytes = &body[cursor..end];
            buf.copy_from_slice(bytes);

            String::from_utf8(buf.to_vec())?
        };

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
    for transaction in transactions {
        writer.write_all(&BIN_HEADER_ID)?;
        writer.write_all(&calculate_body_size(transaction).to_be_bytes())?;
        writer.write_all(&transaction.tx_id.to_be_bytes())?;
        writer.write_all(&[transaction.tx_type.as_bytes()])?;
        writer.write_all(&transaction.from_user_id.to_be_bytes())?;
        writer.write_all(&transaction.to_user_id.to_be_bytes())?;
        writer.write_all(&transaction.amount.to_be_bytes())?;
        writer.write_all(&transaction.timestamp.to_be_bytes())?;
        writer.write_all(&[transaction.status.as_bytes()])?;
        writer.write_all(&transaction.description.len().to_be_bytes())?;
        writer.write_all(transaction.description.as_bytes())?;
    }

    writer.flush()?;
    Ok(())
}

fn calculate_body_size(transaction: &Transaction) -> usize {
    let size: usize = transaction.tx_id.to_be_bytes().len()
        + transaction.tx_type.as_bytes().to_be_bytes().len()
        + transaction.from_user_id.to_be_bytes().len()
        + transaction.to_user_id.to_be_bytes().len()
        + transaction.amount.to_be_bytes().len()
        + transaction.timestamp.to_be_bytes().len()
        + transaction.status.as_bytes().to_be_bytes().len()
        + transaction.description.len();

    size
}

fn parse_raw_bytes_to_u64_be(source: &[u8], cursor: &mut usize) -> u64 {
    let end = *cursor + 8;
    let bytes = &source[*cursor..end];
    let mut array = [0u8; 8];
    array.copy_from_slice(bytes);
    *cursor += 8;
    u64::from_be_bytes(array)
}

fn parse_raw_bytes_to_u8_be(source: &[u8], cursor: &mut usize) -> u8 {
    let end = *cursor + 1;
    let bytes = &source[*cursor..end];
    let mut array = [0u8; 1];
    array.copy_from_slice(bytes);
    *cursor += 1;
    u8::from_be_bytes(array)
}

fn parse_raw_bytes_to_i64_be(source: &[u8], cursor: &mut usize) -> i64 {
    let end = *cursor + 8;
    let bytes = &source[*cursor..end];
    let mut array = [0u8; 8];
    array.copy_from_slice(bytes);
    *cursor += 8;
    i64::from_be_bytes(array)
}

fn parse_raw_bytes_to_u32_be(source: &[u8], cursor: &mut usize) -> u32 {
    let end = *cursor + 4;
    let bytes = &source[*cursor..end];
    let mut array = [0u8; 4];
    array.copy_from_slice(bytes);
    *cursor += 4;
    u32::from_be_bytes(array)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_success(){}

    #[test]
    fn test_read_failure(){}

    #[test]
    fn test_write_success(){}

    #[test]
    fn test_write_failure(){}

    #[test]
    fn test_calculate_body_size(){}
    
    #[test]
    fn test_parse_raw_bytes_to_u64_be(){
        let test_value = 100u64;
        let test_buffer = test_value.to_be_bytes();
        let mut cursor = 0usize;
        let result = parse_raw_bytes_to_u64_be(&test_buffer, &mut cursor);
        assert_eq!(result, test_value);
    }
    #[test]
    fn test_parse_raw_bytes_to_u8_be(){
        let test_value = 100u8;
        let test_buffer = test_value.to_be_bytes();
        let mut cursor = 0usize;
        let result = parse_raw_bytes_to_u8_be(&test_buffer, &mut cursor);
        assert_eq!(result, test_value);
    }
    #[test]
    fn test_parse_raw_bytes_to_i64_be(){
        let test_value = 100i64;
        let test_buffer = test_value.to_be_bytes();
        let mut cursor = 0usize;
        let result = parse_raw_bytes_to_i64_be(&test_buffer, &mut cursor);
        assert_eq!(result, test_value);
    }
    #[test]
    fn test_parse_raw_bytes_to_u32_be(){
        let test_value = 100u32;
        let test_buffer = test_value.to_be_bytes();
        let mut cursor = 0usize;
        let result = parse_raw_bytes_to_u32_be(&test_buffer, &mut cursor);
        assert_eq!(result, test_value);
    }
}