# Rust Parsing Benchmarks

This repo tries to assess Rust parsing performance.

| crate      | parser type   | action code | integration        | input type              | precedence climbing | parameterized rules | streaming input |
|------------|---------------|-------------|--------------------|-------------------------|---------------------|---------------------|-----------------|
| [chumsky]  | combinators   | in source   | library            | `&str`                  | ?                   | ?                   | ?               |
| [combine]  | combinators   | in source   | library            | `&str`                  | ?                   | ?                   | ?               |
| [grmtools] | CFG           | in grammar  | library            | ?                       | ?                   | ?                   | ?               |
| [lalrpop]  | LR(1)         | in grammar  | build script       | `&str`                  | No                  | Yes                 | No              |
| [lelwel]   | LL(1)         | in grammar  | build script       | `&str`                  | Yes                 | Yes                 | No              |
| [logos]    | lexer         | in source   | proc macro         | `&str`, `&[u8]`         | ?                   | ?                   | ?               |
| [nom]      | combinators   | in source   | library            | `&str`, `&[u8]`, custom | No                  | Yes                 | Yes             |
| [parol]    | LL(k)/LALR(R) | in grammar  | build script       | `&str`                  | No                  | ?                   | No              |
| [peg]      | PEG           | in grammar  | proc macro (block) | `&str`, `&[T]`, custom  | Yes                 | Yes                 | No              |
| [pest]     | PEG           | external    | proc macro (file)  | `&str`                  | Yes                 | No                  | No              |
| [winnow]   | combinators   | in source   | library            | `&str`, `&[T]`, custom  | No                  | Yes                 | Yes             |
| [yap]      | combinators   | in source   | library            | `&str`, `&[T]`, custom  | No                  | Yes                 | ?               |

Formerly, we compared:
- [pom]: lack of notoriety

# Results

Name | Overhead (release) | Build (debug) | Parse (release) | Downloads | Version
-----|--------------------|---------------|-----------------|-----------|--------
null | 0 KiB | 194ms | 4ms | - | -
grmtools | 2,573 KiB | 12s | 167ms | ![Download count](https://img.shields.io/crates/dr/cfgrammar) | v0.13.10
chumsky | 561 KiB | 6s | 335ms | ![Download count](https://img.shields.io/crates/dr/chumsky) | v0.9.3
combine | 184 KiB | 4s | 47ms | ![Download count](https://img.shields.io/crates/dr/combine) | v3.8.1
lalrpop | 1,526 KiB | 12s | 37ms | ![Download count](https://img.shields.io/crates/dr/lalrpop-util) | v0.22.1
lelwel | 152 KiB | 4s | 10ms | ![Download count](https://img.shields.io/crates/dr/lelwel) | v0.12.0
logos | 90 KiB | 5s | 20ms | ![Download count](https://img.shields.io/crates/dr/logos) | v0.15.0
nom | 98 KiB | 3s | 59ms | ![Download count](https://img.shields.io/crates/dr/nom) | v8.0.0
parol | 1,718 KiB | 14s | 264ms | ![Download count](https://img.shields.io/crates/dr/parol_runtime) | v3.0.0
peg | 85 KiB | 2s | 21ms | ![Download count](https://img.shields.io/crates/dr/peg) | v0.8.4
pest | 130 KiB | 4s | 54ms | ![Download count](https://img.shields.io/crates/dr/pest) | v2.7.15
serde_json | 55 KiB | 3s | 14ms | ![Download count](https://img.shields.io/crates/dr/serde_json) | v1.0.139
winnow | 79 KiB | 2s | 27ms | ![Download count](https://img.shields.io/crates/dr/winnow) | v0.7.3
yap | 65 KiB | 447ms | 31ms | ![Download count](https://img.shields.io/crates/dr/yap) | v0.12.0

*System: Linux 5.4.0-170-generic (x86_64), rustc 1.85.1 (4eb161250 2025-03-15) w/ `-j 8`*

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
[lelwel]: https://github.com/0x2a-42/lelwel
[logos]: https://github.com/maciejhirsz/logos
[nom]: https://github.com/geal/nom
[parol]: https://github.com/jsinger67/parol
[peg]: https://github.com/kevinmehall/rust-peg
[pest]: https://github.com/pest-parser/pest
[pom]: https://github.com/j-f-liu/pom
[winnow]: https://github.com/winnow-rs/winnow
[yap]: https://github.com/jsdw/yap
[yap]: https://github.com/jsdw/yap
[grmtools]: https://crates.io/crates/cfgrammar
