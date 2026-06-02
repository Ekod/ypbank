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

        let tx_id = parse_raw_bytes_to_u64_be(&body, &mut cursor)?;
        let tx_type = TransactionType::from(parse_raw_bytes_to_u8_be(&body, &mut cursor)?);
        let from_user_id = parse_raw_bytes_to_u64_be(&body, &mut cursor)?;
        let to_user_id = parse_raw_bytes_to_u64_be(&body, &mut cursor)?;
        let amount = parse_raw_bytes_to_i64_be(&body, &mut cursor)?;
        let timestamp = parse_raw_bytes_to_u64_be(&body, &mut cursor)?;
        let status = TransactionStatus::from(parse_raw_bytes_to_u8_be(&body, &mut cursor)?);
        let description_length = parse_raw_bytes_to_u32_be(&body, &mut cursor)?;
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
    transaction.tx_id.to_be_bytes().len()
        + transaction.tx_type.as_bytes().to_be_bytes().len()
        + transaction.from_user_id.to_be_bytes().len()
        + transaction.to_user_id.to_be_bytes().len()
        + transaction.amount.to_be_bytes().len()
        + transaction.timestamp.to_be_bytes().len()
        + transaction.status.as_bytes().to_be_bytes().len()
        + size_of::<u32>()// размер поля в котором хранится информация по размеру поля description
        + transaction.description.len()
}

fn is_cursor_range_valid(source: &[u8], cursor: usize, end_cursor: usize) -> bool {
    if source.get(cursor).is_none() {
        return false;
    }

    if source.get(end_cursor - 1).is_none() {
        // -1 так как нам надо проверить не включая указанный индекс
        return false;
    }

    true
}

fn parse_raw_bytes_to_u64_be(source: &[u8], cursor: &mut usize) -> Result<u64, ParserError> {
    let end = *cursor + 8;
    if !is_cursor_range_valid(source, *cursor, end) {
        return Err(ParserError::InvalidRange(
            "курсор выходит за пределы буффера".to_string(),
        ));
    }
    let bytes = &source[*cursor..end];
    let mut array = [0u8; 8];
    array.copy_from_slice(bytes);
    *cursor += 8;
    Ok(u64::from_be_bytes(array))
}

fn parse_raw_bytes_to_u8_be(source: &[u8], cursor: &mut usize) -> Result<u8, ParserError> {
    let end = *cursor + 1;
    if !is_cursor_range_valid(source, *cursor, end) {
        return Err(ParserError::InvalidRange(
            "курсор выходит за пределы буффера".to_string(),
        ));
    }
    let bytes = &source[*cursor..end];
    let mut array = [0u8; 1];
    array.copy_from_slice(bytes);
    *cursor += 1;
    Ok(u8::from_be_bytes(array))
}

fn parse_raw_bytes_to_i64_be(source: &[u8], cursor: &mut usize) -> Result<i64, ParserError> {
    let end = *cursor + 8;
    if !is_cursor_range_valid(source, *cursor, end) {
        return Err(ParserError::InvalidRange(
            "курсор выходит за пределы буффера".to_string(),
        ));
    }
    let bytes = &source[*cursor..end];
    let mut array = [0u8; 8];
    array.copy_from_slice(bytes);
    *cursor += 8;
    Ok(i64::from_be_bytes(array))
}

fn parse_raw_bytes_to_u32_be(source: &[u8], cursor: &mut usize) -> Result<u32, ParserError> {
    let end = *cursor + 4;
    if !is_cursor_range_valid(source, *cursor, end) {
        return Err(ParserError::InvalidRange(
            "курсор выходит за пределы буффера".to_string(),
        ));
    }
    let bytes = &source[*cursor..end];
    let mut array = [0u8; 4];
    array.copy_from_slice(bytes);
    *cursor += 4;
    Ok(u32::from_be_bytes(array))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::RngExt;
    use std::fs::{File, remove_file};
    use std::path::Path;

    #[test]
    fn test_read_success() {
        let r = rand::rng().random::<u64>();
        let test_file_name = r.to_string() + "test.bin";
        let mut file = File::create(&test_file_name).unwrap();
        let transaction = Transaction {
            tx_id: 10,
            tx_type: TransactionType::Deposit,
            from_user_id: 1,
            to_user_id: 2,
            amount: 100,
            timestamp: 500,
            status: TransactionStatus::Success,
            description: "description".to_string(),
        };

        let body_size = transaction.tx_id.to_be_bytes().len()
            + transaction.tx_type.as_bytes().to_be_bytes().len()
            + transaction.from_user_id.to_be_bytes().len()
            + transaction.to_user_id.to_be_bytes().len()
            + transaction.amount.to_be_bytes().len()
            + transaction.timestamp.to_be_bytes().len()
            + transaction.status.as_bytes().to_be_bytes().len()
            + size_of::<u32>()// размер поля в котором хранится информация по размеру поля description
            + transaction.description.len();

        let description_size = transaction.description.as_bytes().len() as u32;

        file.write_all(&BIN_HEADER_ID).unwrap();
        file.write_all(&(body_size as u32).to_be_bytes()).unwrap();
        file.write_all(&transaction.tx_id.to_be_bytes()).unwrap();
        file.write_all(&[transaction.tx_type.as_bytes()]).unwrap();
        file.write_all(&transaction.from_user_id.to_be_bytes())
            .unwrap();
        file.write_all(&transaction.to_user_id.to_be_bytes())
            .unwrap();
        file.write_all(&transaction.amount.to_be_bytes()).unwrap();
        file.write_all(&transaction.timestamp.to_be_bytes())
            .unwrap();
        file.write_all(&[transaction.status.as_bytes()]).unwrap();
        file.write_all(&description_size.to_be_bytes()).unwrap();
        file.write_all(transaction.description.as_bytes()).unwrap();
        file.flush().unwrap();

        let mut reader = File::open(Path::new(&test_file_name)).unwrap();

        let result = match read(&mut reader) {
            Ok(result) => result,
            Err(error) => panic!("возникла проблема при чтении файла {:?}", error),
        };

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], transaction);
        remove_file(Path::new(&test_file_name)).unwrap();
    }

    #[test]
    fn test_read_failure_length_zero() {
        let r = rand::rng().random::<u64>();
        let test_file_name = String::from(r.to_string() + "test.bin");
        let _ = File::create(&test_file_name).unwrap();
        let mut reader = File::open(Path::new(&test_file_name)).unwrap();

        let result = match read(&mut reader) {
            Ok(result) => result,
            Err(error) => panic!("возникла проблема при чтении файла {:?}", error),
        };

        assert_eq!(result.len(), 0);
        remove_file(Path::new(&test_file_name)).unwrap();
    }

    #[test]
    fn test_read_failure_no_header() {
        let r = rand::rng().random::<u64>();
        let test_file_name = String::from(r.to_string() + "test.bin");
        let mut file = File::create(&test_file_name).unwrap();
        let transaction = Transaction {
            tx_id: 10,
            tx_type: TransactionType::Deposit,
            from_user_id: 1,
            to_user_id: 2,
            amount: 100,
            timestamp: 500,
            status: TransactionStatus::Success,
            description: "description".to_string(),
        };

        let body_size = transaction.tx_id.to_be_bytes().len()
            + transaction.tx_type.as_bytes().to_be_bytes().len()
            + transaction.from_user_id.to_be_bytes().len()
            + transaction.to_user_id.to_be_bytes().len()
            + transaction.amount.to_be_bytes().len()
            + transaction.timestamp.to_be_bytes().len()
            + transaction.status.as_bytes().to_be_bytes().len()
            + size_of::<u32>()// размер поля в котором хранится информация по размеру поля description
            + transaction.description.len();

        let description_size = transaction.description.as_bytes().len() as u32;

        file.write_all(&(body_size as u32).to_be_bytes()).unwrap();
        file.write_all(&transaction.tx_id.to_be_bytes()).unwrap();
        file.write_all(&[transaction.tx_type.as_bytes()]).unwrap();
        file.write_all(&transaction.from_user_id.to_be_bytes())
            .unwrap();
        file.write_all(&transaction.to_user_id.to_be_bytes())
            .unwrap();
        file.write_all(&transaction.amount.to_be_bytes()).unwrap();
        file.write_all(&transaction.timestamp.to_be_bytes())
            .unwrap();
        file.write_all(&[transaction.status.as_bytes()]).unwrap();
        file.write_all(&description_size.to_be_bytes()).unwrap();
        file.write_all(transaction.description.as_bytes()).unwrap();
        file.flush().unwrap();

        let mut reader = File::open(Path::new(&test_file_name)).unwrap();

        let result = match read(&mut reader) {
            Ok(result) => result,
            Err(error) => panic!("возникла проблема при чтении файла {:?}", error),
        };

        assert_eq!(result.len(), 0);
        remove_file(Path::new(&test_file_name)).unwrap();
    }

    #[test]
    fn test_read_failure_wrong_buffer_size() {
        let r = rand::rng().random::<u64>();
        let test_file_name = String::from(r.to_string() + "test.bin");
        let mut file = File::create(&test_file_name).unwrap();
        file.write_all(&BIN_HEADER_ID).unwrap();

        let mut reader = File::open(Path::new(&test_file_name)).unwrap();

        if let Err(err) = read(&mut reader) {
            assert_eq!(
                err,
                ParserError::InvalidRange("курсор выходит за пределы буффера".to_string(),)
            );
        };

        remove_file(Path::new(&test_file_name)).unwrap();
    }

    #[test]
    fn test_write_success() {
        let r = rand::rng().random::<u64>();
        let test_file_name = String::from(r.to_string() + "test.bin");
        let mut file = File::create(&test_file_name).unwrap();
        let transaction = Transaction {
            tx_id: 10,
            tx_type: TransactionType::Deposit,
            from_user_id: 1,
            to_user_id: 2,
            amount: 100,
            timestamp: 500,
            status: TransactionStatus::Success,
            description: "description".to_string(),
        };

        let input = vec![transaction];

        let result = write(&mut file, &input);

        assert!(result.is_ok());
        remove_file(Path::new(&test_file_name)).unwrap();
    }

    #[test]
    fn test_calculate_body_size() {
        let transaction = Transaction {
            tx_id: 10,
            tx_type: TransactionType::Deposit,
            from_user_id: 1,
            to_user_id: 2,
            amount: 100,
            timestamp: 500,
            status: TransactionStatus::Success,
            description: "description".to_string(),
        };

        let body_size = transaction.tx_id.to_be_bytes().len()
            + transaction.tx_type.as_bytes().to_be_bytes().len()
            + transaction.from_user_id.to_be_bytes().len()
            + transaction.to_user_id.to_be_bytes().len()
            + transaction.amount.to_be_bytes().len()
            + transaction.timestamp.to_be_bytes().len()
            + transaction.status.as_bytes().to_be_bytes().len()
            + size_of::<u32>()// размер поля в котором хранится информация по размеру поля description
            + transaction.description.len();

        let result = calculate_body_size(&transaction);

        assert_eq!(body_size, result);
    }

    #[test]
    fn test_parse_raw_bytes_to_u64_be_success() {
        let test_value = 100u64;
        let test_buffer = test_value.to_be_bytes();
        let mut cursor = 0usize;
        if let Ok(result) = parse_raw_bytes_to_u64_be(&test_buffer, &mut cursor) {
            assert_eq!(result, test_value);
            return;
        };
        panic!("test has failed");
    }
    #[test]
    fn test_parse_raw_bytes_to_u8_be_success() {
        let test_value = 100u8;
        let test_buffer = test_value.to_be_bytes();
        let mut cursor = 0usize;
        if let Ok(result) = parse_raw_bytes_to_u8_be(&test_buffer, &mut cursor) {
            assert_eq!(result, test_value);
            return;
        };
        panic!("test has failed");
    }
    #[test]
    fn test_parse_raw_bytes_to_i64_be_success() {
        let test_value = 100i64;
        let test_buffer = test_value.to_be_bytes();
        let mut cursor = 0usize;
        if let Ok(result) = parse_raw_bytes_to_i64_be(&test_buffer, &mut cursor) {
            assert_eq!(result, test_value);
            return;
        };
        panic!("test has failed");
    }
    #[test]
    fn test_parse_raw_bytes_to_u32_be_success() {
        let test_value = 100u32;
        let test_buffer = test_value.to_be_bytes();
        let mut cursor = 0usize;
        if let Ok(result) = parse_raw_bytes_to_u32_be(&test_buffer, &mut cursor) {
            assert_eq!(result, test_value);
            return;
        };
        panic!("test has failed");
    }
}
