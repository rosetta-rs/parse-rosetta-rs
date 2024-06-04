# Rust Parsing Benchmarks

This repo tries to assess Rust parsing performance.

| crate     | parser type | action code | integration        | input type             | precedence climbing | parameterized rules | streaming input |
|-----------|-------------|-------------|--------------------|------------------------|---------------------|---------------------|-----------------|
| [chumsky] | combinators | in source   | library            | `&str`                 | ?                   | ?                   | ?               |
| [combine] | combinators | in source   | library            | `&str`                 | ?                   | ?                   | ?               |
| [lalrpop] | LR(1)       | in grammar  | build script       | `&str`                 | No                  | Yes                 | No              |
| [logos]   | lexer       | in source   | proc macro         | `&str`, `&[u8]`        | ?                   | ?                   | ?               |
| [nom]     | combinators | in source   | library            | `&[u8]`, custom        | No                  | Yes                 | Yes             |
| [peg]     | PEG         | in grammar  | proc macro (block) | `&str`, `&[T]`, custom | Yes                 | Yes                 | No              |
| [pest]    | PEG         | external    | proc macro (file)  | `&str`                 | Yes                 | No                  | No              |
| [winnow]  | combinators | in source   | library            | `&str`, `&[T]`, custom | No                  | Yes                 | Yes             |
| [yap]     | combinators | in source   | library            | `&str`, `&[T]`, custom | No                  | Yes                 | ?               |

Formerly, we compared:
- [pom]: lack of notoriety

# Results

Name | Overhead (release) | Build (debug) | Parse (release) | Downloads | Version
-----|--------------------|---------------|-----------------|-----------|--------
null | 0 KiB | 199ms | 4ms | - | -
chumsky | 627 KiB | 6s | 370ms | ![Download count](https://img.shields.io/crates/dr/ariadne) | v0.4.1
logos | 172 KiB | 5s | 17ms | ![Download count](https://img.shields.io/crates/dr/ariadne) | v0.4.1
grmtools | 2,520 KiB | 14s | 162ms | ![Download count](https://img.shields.io/crates/dr/cfgrammar) | v0.13.6
combine | 182 KiB | 4s | 48ms | ![Download count](https://img.shields.io/crates/dr/combine) | v3.8.1
lalrpop | 1,487 KiB | 12s | 36ms | ![Download count](https://img.shields.io/crates/dr/lalrpop-util) | v0.20.2
nom | 94 KiB | 2s | 69ms | ![Download count](https://img.shields.io/crates/dr/nom) | v7.1.3
peg | 71 KiB | 2s | **invalid** | ![Download count](https://img.shields.io/crates/dr/peg) | v0.8.3
pest | 121 KiB | 4s | 56ms | ![Download count](https://img.shields.io/crates/dr/pest) | v2.7.10
serde_json | 45 KiB | 3s | 13ms | ![Download count](https://img.shields.io/crates/dr/serde_json) | v1.0.117
winnow | 70 KiB | 2s | 22ms | ![Download count](https://img.shields.io/crates/dr/winnow) | v0.6.9
yap | 55 KiB | 463ms | 32ms | ![Download count](https://img.shields.io/crates/dr/yap) | v0.12.0

*System: Linux 5.4.0-170-generic (x86_64), rustc 1.78.0 (9b00956e5 2024-04-29) w/ `-j 8`*

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
