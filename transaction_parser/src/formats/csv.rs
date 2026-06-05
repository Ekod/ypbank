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
                        "csv заголовок не валидный".to_string(),
                    ));
                }
            }
            Err(err) => {
                return Err(ParserError::InvalidHeader(err.to_string()));
            }
        },
        None => {
            return Err(ParserError::InvalidHeader(
                "заголовок не найден".to_string(),
            ));
        }
    }

    let _ = lines.remove(0);

    for line in lines {
        let line = line.expect("не удалось прочитать строку");
        if line.is_empty() {
            continue;
        }

        let transaction_fields = line.split(",").collect::<Vec<&str>>();
        if transaction_fields.len() != 8 {
            return Err(ParserError::InvalidRecord(
                "неверная длина транзакции".to_string(),
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
    if transactions.is_empty() {
        return Err(ParserError::EmptyTransactions("транзакций нет".to_string()));
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{TransactionStatus, TransactionType};
    use rand::RngExt;
    use std::fs::{File, remove_file};
    use std::path::Path;
    #[test]
    fn test_read_success() {
        let r = rand::rng().random::<u64>();
        let test_file_name = String::from(r.to_string() + "test.csv");
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
        writeln!(file, "{}", CSV_HEADER).unwrap();
        writeln!(
            file,
            "{},{},{},{},{},{},{},{}",
            transaction.tx_id,
            transaction.tx_type,
            transaction.from_user_id,
            transaction.to_user_id,
            transaction.amount,
            transaction.timestamp,
            transaction.status,
            transaction.description,
        )
        .unwrap();
        file.flush().unwrap();

        let mut reader = File::open(Path::new(&test_file_name)).unwrap();

        let result = match read(&mut reader) {
            Ok(result) => result,
            Err(error) => panic!("возникла проблема при чтении файла {:?}", error),
        };

        remove_file(Path::new(&test_file_name)).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], transaction);
    }

    #[test]
    fn test_read_failure_header_not_found() {
        let r = rand::rng().random::<u64>();
        let test_file_name = String::from(r.to_string() + "test.csv");
        let _ = File::create(&test_file_name).unwrap();

        let mut reader = File::open(Path::new(&test_file_name)).unwrap();
        match read(&mut reader) {
            Ok(_) => {
                remove_file(Path::new(&test_file_name)).unwrap();
                panic!("test has failed");
            }
            Err(err) => {
                remove_file(Path::new(&test_file_name)).unwrap();
                assert_eq!(
                    err,
                    ParserError::InvalidHeader("заголовок не найден".to_string())
                );
            }
        }
    }

    #[test]
    fn test_read_failure_header_is_not_valid() {
        let r = rand::rng().random::<u64>();
        let test_file_name = String::from(r.to_string() + "test.csv");
        let mut file = File::create(&test_file_name).unwrap();

        writeln!(file, "Some header").unwrap();

        let mut reader = File::open(Path::new(&test_file_name)).unwrap();
        match read(&mut reader) {
            Ok(_) => {
                remove_file(Path::new(&test_file_name)).unwrap();
                panic!("test has failed");
            }
            Err(err) => {
                remove_file(Path::new(&test_file_name)).unwrap();
                assert_eq!(
                    err,
                    ParserError::InvalidHeader("csv заголовок не валидный".to_string(),)
                );
            }
        }
    }

    #[test]
    fn test_read_failure_wrong_transaction_length() {
        let r = rand::rng().random::<u64>();
        let test_file_name = String::from(r.to_string() + "test.csv");
        let mut file = File::create(&test_file_name).unwrap();

        writeln!(file, "{}", CSV_HEADER).unwrap();
        writeln!(file, "transaction").unwrap();
        file.flush().unwrap();

        let mut reader = File::open(Path::new(&test_file_name)).unwrap();

        match read(&mut reader) {
            Ok(_) => {
                remove_file(Path::new(&test_file_name)).unwrap();
                panic!("test has failed");
            }
            Err(err) => {
                remove_file(Path::new(&test_file_name)).unwrap();
                assert_eq!(
                    err,
                    ParserError::InvalidRecord("неверная длина транзакции".to_string(),)
                )
            }
        }
    }

    #[test]
    fn test_write_success() {
        let r = rand::rng().random::<u64>();
        let test_file_name = String::from(r.to_string() + "test.csv");
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
        writeln!(file, "{}", CSV_HEADER).unwrap();
        writeln!(
            file,
            "{},{},{},{},{},{},{},{}",
            transaction.tx_id,
            transaction.tx_type,
            transaction.from_user_id,
            transaction.to_user_id,
            transaction.amount,
            transaction.timestamp,
            transaction.status,
            transaction.description,
        )
        .unwrap();
        let input = vec![transaction];

        let result = write(&mut file, &input);

        remove_file(Path::new(&test_file_name)).unwrap();
        assert!(result.is_ok());
    }

    #[test]
    fn test_write_failure() {
        let r = rand::rng().random::<u64>();
        let test_file_name = String::from(r.to_string() + "test.csv");
        let mut file = File::create(&test_file_name).unwrap();

        match write(&mut file, &vec![]) {
            Ok(_) => {
                remove_file(Path::new(&test_file_name)).unwrap();
                panic!("test has failed");
            }
            Err(err) => {
                remove_file(Path::new(&test_file_name)).unwrap();
                assert_eq!(
                    err,
                    ParserError::EmptyTransactions("транзакций нет".to_string()),
                )
            }
        }
    }
}
