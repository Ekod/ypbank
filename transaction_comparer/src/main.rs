mod errors;

use crate::errors::ComparerError;
use std::env;
use std::fs::File;
use std::path::Path;
use transaction_parser::read_from_file_to_transactions;
use transaction_parser::types::Transaction;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let _ = args.remove(0); // Убираем инфу про источник запуска
    println!("{:?}", args);
    if args.len() < 8 {
        eprintln!("Использование:");
        eprintln!("  --file1 <file_1>");
        eprintln!("  --input-format1 <format1> (csv/bin/txt)");
        eprintln!("  --file2 <file_2>");
        eprintln!("  --input-format2 <format2> (csv/bin/txt)");
        return;
    }

    let mut in_file_1 = String::new();
    let mut input_format_1 = String::new();
    let mut in_file_2 = String::new();
    let mut input_format_2 = String::new();

    while !args.is_empty() {
        match args.remove(0).as_str() {
            "--file1" => {
                in_file_1 = args.remove(0);
            }
            "--input-format1" => {
                input_format_1 = args.remove(0);
            }
            "--file2" => {
                in_file_2 = args.remove(0);
            }
            "--input-format2" => {
                input_format_2 = args.remove(0);
            }
            arg => {
                eprintln!("неизвестная команда: {}", arg);
                return;
            }
        }
    }

    if in_file_1.is_empty() {
        eprintln!("не указан входной файл 1");
        return;
    }

    if !Path::new(&in_file_1).exists() {
        eprintln!("файл {} не существует", in_file_1);
        return;
    }

    if input_format_1.is_empty() {
        eprintln!("не указан входной формат 1");
        return;
    }

    if in_file_2.is_empty() {
        eprintln!("не указан входной файл 2");
        return;
    }

    if !Path::new(&in_file_2).exists() {
        eprintln!("файл {} не существует", in_file_2);
        return;
    }

    if input_format_2.is_empty() {
        eprintln!("не указан входной формат 2");
        return;
    }

    let mapped_in_format_1 = match map_format(input_format_1.as_str()) {
        Ok(format) => format,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let mapped_in_format_2 = match map_format(input_format_2.as_str()) {
        Ok(format) => format,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let mut file_1 = match File::open(&in_file_1) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("не удалось открыть файл 1. Ошибка: {}", e);
            return;
        }
    };

    let mut file_2 = match File::open(&in_file_2) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("не удалось открыть файл 2. Ошибка: {}", e);
            return;
        }
    };

    let transactions_1 = match read_from_file_to_transactions(&mut file_1, mapped_in_format_1) {
        Ok(transactions) => transactions,
        Err(e) => {
            eprintln!("ошибка при обработке файла 1: {}", e);
            return;
        }
    };
    let transactions_2 = match read_from_file_to_transactions(&mut file_2, mapped_in_format_2) {
        Ok(transactions) => transactions,
        Err(e) => {
            eprintln!("ошибка при обработке файла 2: {}", e);
            return;
        }
    };

    let (are_equal, unequal_transaction_ids) =
        are_transactions_equal(&transactions_1, &transactions_2);
    if are_equal {
        println!(
            "# Вывод: транзакции в файлах '{}' и '{}' идентичны",
            &in_file_1, &in_file_2
        );
    } else {
        match unequal_transaction_ids {
            Some(unequal_transaction_ids) => {
                eprintln!("файлы {} и {} имеют разные записи", &in_file_1, &in_file_2);
                eprintln!("TX_ID записей, которые различаются");
                for id in unequal_transaction_ids {
                    eprintln!("{} {}", id.0, id.1);
                }
            }
            None => {
                eprintln!(
                    "файлы {} и {} имеют разное количество записей",
                    &in_file_1, &in_file_2
                );
            }
        }
    }
}

fn map_format(format: &str) -> Result<transaction_parser::types::Format, ComparerError> {
    match format {
        "csv" => Ok(transaction_parser::types::Format::Csv),
        "bin" | "binary" => Ok(transaction_parser::types::Format::Binary),
        "txt" | "text" => Ok(transaction_parser::types::Format::Text),
        _ => Err(ComparerError::UnknownFormat(
            "указанный формат не поддерживается".to_string(),
        )),
    }
}

fn are_transactions_equal(
    transactions_1: &[Transaction],
    transactions_2: &[Transaction],
) -> (bool, Option<Vec<(u64, u64)>>) {
    if transactions_1.len() != transactions_2.len() {
        return (false, None);
    }

    let mut unequal_transaction_ids = Vec::new();

    let mut idx: usize = 0;

    // В данной версии сравнение идёт не только по данным в самих записях,
    // но и то, что транзакции в обоих векторах идут в одиннаковом порядке.
    // Если при проверке на идентичность не нужно проверять порядок, а только данные, то переделаю
    while idx < transactions_1.len() {
        let transaction1 = &transactions_1[idx];
        let transaction2 = &transactions_2[idx];

        if transaction1 != transaction2 {
            unequal_transaction_ids.push((transaction1.tx_id, transaction2.tx_id));
        }

        idx += 1;
    }

    if !unequal_transaction_ids.is_empty() {
        (false, Some(unequal_transaction_ids))
    } else {
        (true, None)
    }
}
