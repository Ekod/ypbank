# YPComparer

Сравнивает на идентичность 2 файла с транзакциями.
****Важное замечание**** - на данный момент сравниваются не только данные в транзакциях,
но и порядок транзакций в файлах! Иными словами, если у вас 2 транзакции имеют одиннаковые данные,
но идут в разном порядке в файлах, то YPComparer скажет, что файлы не идентичны.

# Как запустить

В корне проекта запустить команду
cargo run --bin transaction_comparer -- --file1 records_example.csv --input-format1 csv --file2 records_example.csv
--input-format2 csv
где вместо records_example.csv - ваши файлы, вместо csv - форматы ваших файлов.

# Примеры

Список тестовых команд:

```terminaloutput
cargo run --bin transaction_comparer -- --file1 ../records_example.csv --input-format1 csv --file2 ../records_example.csv --input-format2 csv"
cargo run --bin transaction_comparer -- --file1 ../records_example.csv --input-format1 csv --file2 ../records_example.txt --input-format2 txt"
cargo run --bin transaction_comparer -- --file1 ../records_example.csv --input-format1 csv --file2 ../records_example.bin --input-format2 bin"
cargo run --bin transaction_comparer -- --file1 ../records_example.txt --input-format1 txt --file2 ../records_example.txt --input-format2 txt"
cargo run --bin transaction_comparer -- --file1 ../records_example.txt --input-format1 txt --file2 ../records_example.csv --input-format2 csv"
cargo run --bin transaction_comparer -- --file1 ../records_example.txt --input-format1 txt --file2 ../records_example.bin --input-format2 bin"
cargo run --bin transaction_comparer -- --file1 ../records_example.bin --input-format1 bin --file2 ../records_example.bin --input-format2 bin"
cargo run --bin transaction_comparer -- --file1 ../records_example.bin --input-format1 bin --file2 ../records_example.txt --input-format2 txt"
cargo run --bin transaction_comparer -- --file1 ../records_example.bin --input-format1 bin --file2 ../records_example.csv --input-format2 csv"
```