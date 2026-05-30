mod errors;

use crate::errors::ConverterError;
use std::fs::File;
use std::path::Path;
use std::{env, io};
use transaction_parser::process_transaction_file;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let _ = args.remove(0); // Убираем инфу про источник запуска
    if args.len() < 6 {
        eprintln!("Использование:");
        eprintln!("  --input <input_file>");
        eprintln!("  --input-format <format> (csv/bin/txt)");
        eprintln!("  --output-format <format> (csv/bin/txt)");
        return;
    }

    let mut in_file = String::new();
    let mut input_format = String::new();
    let mut output_format = String::new();

    while !args.is_empty() {
        match args.remove(0).as_str() {
            "--input" => {
                in_file = args.remove(0);
            }
            "--input-format" => {
                input_format = args.remove(0);
            }
            "--output-format" => {
                output_format = args.remove(0);
            }
            arg => {
                eprintln!("неизвестная команда: {}", arg);
                return;
            }
        }
    }

    if in_file.is_empty() {
        eprintln!("не указан входной файл");
        return;
    }

    if !Path::new(&in_file).exists() {
        eprintln!("файл {} не существует", in_file);
        return;
    }

    if input_format.is_empty() {
        eprintln!("не указан входной формат");
        return;
    }

    if output_format.is_empty() {
        eprintln!("не указан выходной формат");
        return;
    }

    let mapped_in_format = match map_format(input_format.as_str()) {
        Ok(format) => format,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let mapped_out_format = match map_format(output_format.as_str()) {
        Ok(format) => format,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let mut file = match File::open(&in_file) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("не удалось открыть файл. Ошибка: {}", e);
            return;
        }
    };

    let mut os = io::stdout().lock();

    match process_transaction_file(&mut file, &mut os, mapped_in_format, mapped_out_format) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("ошибка при обработке файла: {}", e);
        }
    };
}

fn map_format(format: &str) -> Result<transaction_parser::types::Format, ConverterError> {
    match format {
        "csv" => Ok(transaction_parser::types::Format::Csv),
        "bin" | "binary" => Ok(transaction_parser::types::Format::Binary),
        "txt" | "text" => Ok(transaction_parser::types::Format::Text),
        _ => Err(ConverterError::UnknownFormat(
            "указанный формат не поддерживается".to_string(),
        )),
    }
}
