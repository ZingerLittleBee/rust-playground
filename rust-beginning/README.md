# Rust 编程第一课
> learn more to see https://time.geekbang.org/column/intro/100085301?tab=intro

## httpie

## thumbor

## queryer
### what it is
use the `SQL` query whatever datasource without import `Spark`
### how it work
> core idea: convert datasource like csv json to DataFrame

step
1. parser `SQL` use [sqlparser-rs]("https://github.com/sqlparser-rs/sqlparser-rs")
2. convert `AST` what datasource parsed by sqlparser to [polars']("https://github.com/pola-rs/polars") DataFrame

## queryer-py
see [queryer-py.md](https://github.com/ZingerLittleBee/rust-playground/tree/master/rust-beginning/queryer-py)

## queryer-node
see [queryer-node.md](https://github.com/ZingerLittleBee/rust-playground/tree/master/rust-beginning/queryer-node)
