# YPConverter

Конвертирует файл с транзакциями из одного формата в другой.
****Важное замечание**** - если конвертируете в бинарный формат, то лучше результат действия программы переводить в
файл, так как консоль может не уметь работать с выводом в бинарном формате

То, как должны быть организованы транзакции в рамках каждого конкреного формата - смотреть в
README [YPParser](../transaction_parser/README.md)

# Как запустить

В корне проекта запустить команду
cargo run --bin transaction_converter -- --input records_example.txt --input-format txt --output-format csv
где вместо records_example.txt - ваш файл, вместо txt - формат входного файла, а вместо csv - выходной фомат.

# Примеры

Список тестовых команд:

```terminaloutput
cargo run --bin transaction_converter -- --input ../records_example.txt --input-format txt --output-format csv"
cargo run --bin transaction_converter -- --input ../records_example.txt --input-format txt --output-format bin"
cargo run --bin transaction_converter -- --input ../records_example.csv --input-format csv --output-format txt"
cargo run --bin transaction_converter -- --input ../records_example.csv --input-format csv --output-format bin"
cargo run --bin transaction_converter -- --input ../records_example.bin --input-format bin --output-format csv"
cargo run --bin transaction_converter -- --input ../records_example.bin --input-format bin --output-format txt"
```