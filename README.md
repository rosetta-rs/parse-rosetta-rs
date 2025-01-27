# Rust Parsing Benchmarks

This repo tries to assess Rust parsing performance.

| crate      | parser type | action code | integration        | input type             | precedence climbing | parameterized rules | streaming input |
|------------|-------------|-------------|--------------------|------------------------|---------------------|---------------------|-----------------|
| [chumsky]  | combinators | in source   | library            | `&str`                 | ?                   | ?                   | ?               |
| [combine]  | combinators | in source   | library            | `&str`                 | ?                   | ?                   | ?               |
| [grmtools] | CFG         | in grammar  | library            | ?                      | ?                   | ?                   | ?               |
| [lalrpop]  | LR(1)       | in grammar  | build script       | `&str`                 | No                  | Yes                 | No              |
| [logos]    | lexer       | in source   | proc macro         | `&str`, `&[u8]`        | ?                   | ?                   | ?               |
| [nom]      | combinators | in source   | library            | `&[u8]`, custom        | No                  | Yes                 | Yes             |
| [peg]      | PEG         | in grammar  | proc macro (block) | `&str`, `&[T]`, custom | Yes                 | Yes                 | No              |
| [pest]     | PEG         | external    | proc macro (file)  | `&str`                 | Yes                 | No                  | No              |
| [winnow]   | combinators | in source   | library            | `&str`, `&[T]`, custom | No                  | Yes                 | Yes             |
| [yap]      | combinators | in source   | library            | `&str`, `&[T]`, custom | No                  | Yes                 | ?               |

Formerly, we compared:
- [pom]: lack of notoriety

# Results

Name | Overhead (release) | Build (debug) | Parse (release) | Downloads | Version
-----|--------------------|---------------|-----------------|-----------|--------
null | 0 KiB | 198ms | 4ms | - | -
grmtools | 2,526 KiB | 13s | 162ms | ![Download count](https://img.shields.io/crates/dr/cfgrammar) | v0.13.8
chumsky | 562 KiB | 6s | 331ms | ![Download count](https://img.shields.io/crates/dr/chumsky) | v0.9.3
combine | 184 KiB | 5s | 50ms | ![Download count](https://img.shields.io/crates/dr/combine) | v3.8.1
lalrpop | 1,496 KiB | 13s | 36ms | ![Download count](https://img.shields.io/crates/dr/lalrpop-util) | v0.22.0
logos | 81 KiB | 5s | 17ms | ![Download count](https://img.shields.io/crates/dr/logos) | v0.15.0
nom | 98 KiB | 3s | 60ms | ![Download count](https://img.shields.io/crates/dr/nom) | v8.0.0
peg | 82 KiB | 2s | 21ms | ![Download count](https://img.shields.io/crates/dr/peg) | v0.8.4
pest | 130 KiB | 4s | 54ms | ![Download count](https://img.shields.io/crates/dr/pest) | v2.7.15
serde_json | 55 KiB | 3s | 14ms | ![Download count](https://img.shields.io/crates/dr/serde_json) | v1.0.134
winnow | 80 KiB | 2s | 28ms | ![Download count](https://img.shields.io/crates/dr/winnow) | v0.6.21
yap | 56 KiB | 450ms | 31ms | ![Download count](https://img.shields.io/crates/dr/yap) | v0.12.0

*System: Linux 5.4.0-170-generic (x86_64), rustc 1.84.0 (9fc6b4312 2025-01-07) w/ `-j 8`*

Note:
- For more "Parse (release)" comparisons, see [parser_benchmarks](https://github.com/rust-bakery/parser_benchmarks)
- Parsers have not been validated and might have differing levels of quality ([#5](https://github.com/epage/parse-benchmarks-rs/issues/5))

# Running the Benchmarks

```bash
$ ./bench.py
$ ./format.py
```

[chumsky]: https://github.com/zesterer/chumsky
[combine]: https://github.com/Marwes/combine
[lalrpop]: https://github.com/lalrpop/lalrpop
[logos]: https://github.com/maciejhirsz/logos
[nom]: https://github.com/geal/nom
[peg]: https://github.com/kevinmehall/rust-peg
[pest]: https://github.com/pest-parser/pest
[pom]: https://github.com/j-f-liu/pom
[winnow]: https://github.com/winnow-rs/winnow
[yap]: https://github.com/jsdw/yap
[yap]: https://github.com/jsdw/yap
[grmtools]: https://crates.io/crates/cfgrammar
