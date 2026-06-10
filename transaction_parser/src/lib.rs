use crate::errors::ParserError;
use crate::formats::{binary, csv, txt};
use crate::types::{Format, Transaction};
use std::fs::File;
use std::io::Write;

mod common;
mod errors;
mod formats;
pub mod types;

/// process_transaction_file отвечает за чтение файла с транзакциями и запись результата в переданный источник
/// file - файл с транзакциями на чтение
/// os - источник в который будут записаны прочитанные транзакции, в случае успешного процесса чтения
/// in_format - формат файла с транзакциями переданного в параметр file
/// out_format - формат транзакций в который нужно перевести прочитанные транзакции из файла переданного в параметр file
pub fn process_transaction_file<W: Write>(
    file: &mut File,
    os: &mut W,
    in_format: Format,
    out_format: Format,
) -> Result<(), ParserError> {
    let transactions = read_from_file_to_transactions(file, in_format)?;
    write_transactions_to_out_stream(transactions, out_format, os)?;

    Ok(())
}

/// read_from_file_to_transactions отвечает за чтение файла с транзакциями
/// file - файл с транзакциями на чтение
/// in_format - формат файла с транзакциями переданного в параметр file
pub fn read_from_file_to_transactions(
    file: &mut File,
    in_format: Format,
) -> Result<Vec<Transaction>, ParserError> {
    match in_format {
        Format::Csv => csv::read(file),
        Format::Text => txt::read(file),
        Format::Binary => binary::read(file),
    }
}

/// write_transactions_to_out_stream отвечает за запись списка транзакций в переданный источник
/// transactions - список транзакций, которые нужно записать
/// out_format - формат транзакций в который нужно перевести транзакции из параметра transactions
/// os - источник в который будут записаны транзакции из параметра transactions
pub fn write_transactions_to_out_stream<W: Write>(
    transactions: Vec<Transaction>,
    out_format: Format,
    os: &mut W,
) -> Result<(), ParserError> {
    match out_format {
        Format::Csv => csv::write(os, &transactions),
        Format::Text => txt::write(os, &transactions),
        Format::Binary => binary::write(os, &transactions),
    }
}
