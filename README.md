# Rust Parsing Benchmarks

This repo tries to assess Rust parsing performance.

| crate      | parser type   | action code | integration        | input type              | precedence             | parameterized rules | streaming input |
|------------|---------------|-------------|--------------------|-------------------------|------------------------|---------------------|-----------------|
| [chumsky]  | combinators   | in source   | library            | `&str`, `&[u8]`, custom | [pratt][chumsky-pratt] | Yes                 | Yes             |
| [combine]  | combinators   | in source   | library            | `&str`                  | ?                      | ?                   | ?               |
| [grmtools] | CFG           | in grammar  | library            | ?                       | ?                      | ?                   | ?               |
| [lalrpop]  | LR(1)         | in grammar  | build script       | `&str`                  | none                   | Yes                 | No              |
| [lelwel]   | LL(1)         | in grammar  | build script       | `&str`                  | pratt                  | Yes                 | No              |
| [logos]    | lexer         | in source   | proc macro         | `&str`, `&[u8]`         | ?                      | ?                   | ?               |
| [nom]      | combinators   | in source   | library            | `&str`, `&[u8]`, custom | [pratt][nom-pratt]     | Yes                 | Yes             |
| [parol]    | LL(k)/LALR(1) | in source   | build script       | `&str`                  | climbing               | No                  | No              |
| [peg]      | PEG           | in grammar  | proc macro (block) | `&str`, `&[T]`, custom  | climbing               | Yes                 | No              |
| [pest]     | PEG           | external    | proc macro (file)  | `&str`                  | climbing               | No                  | No              |
| [winnow]   | combinators   | in source   | library            | `&str`, `&[T]`, custom  | none                   | Yes                 | Yes             |
| [yap]      | combinators   | in source   | library            | `&str`, `&[T]`, custom  | none                   | Yes                 | ?               |

Formerly, we compared:
- [pom]: lack of notoriety

# Results

Name | Overhead (release) | Build (debug) | Parse (release) | Downloads | Version
-----|--------------------|---------------|-----------------|-----------|--------
null | 0 KiB | 217ms | 4ms | - | -
grmtools | 2,623 KiB | 12s | 174ms | ![Download count](https://img.shields.io/crates/dr/cfgrammar) | v0.13.10
chumsky | 117 KiB | 4s | 31ms | ![Download count](https://img.shields.io/crates/dr/chumsky) | v0.10.1
combine | 184 KiB | 4s | 50ms | ![Download count](https://img.shields.io/crates/dr/combine) | v3.8.1
lalrpop | 1,527 KiB | 12s | 38ms | ![Download count](https://img.shields.io/crates/dr/lalrpop) | v0.22.1
lelwel | 148 KiB | 5s | 10ms | ![Download count](https://img.shields.io/crates/dr/lelwel) | v0.8.0
logos | 90 KiB | 5s | 21ms | ![Download count](https://img.shields.io/crates/dr/logos) | v0.15.0
nom | 94 KiB | 3s | 65ms | ![Download count](https://img.shields.io/crates/dr/nom) | v8.0.0
parol | 1,718 KiB | 14s | 239ms | ![Download count](https://img.shields.io/crates/dr/parol) | v3.0.1
peg | 84 KiB | 2s | 22ms | ![Download count](https://img.shields.io/crates/dr/peg) | v0.8.5
pest | 130 KiB | 6s | 62ms | ![Download count](https://img.shields.io/crates/dr/pest) | v2.8.0
serde_json | 59 KiB | 3s | 15ms | ![Download count](https://img.shields.io/crates/dr/serde_json) | v1.0.140
winnow | 75 KiB | 2s | 29ms | ![Download count](https://img.shields.io/crates/dr/winnow) | v0.7.8
yap | 65 KiB | 499ms | 33ms | ![Download count](https://img.shields.io/crates/dr/yap) | v0.12.0

*System: Linux 6.8.0-58-generic (x86_64), rustc 1.86.0 (05f9846f8 2025-03-31) w/ `-j 8`*

Note:
- For more "Parse (release)" comparisons, see [parser_benchmarks](https://github.com/rust-bakery/parser_benchmarks)
- Parsers have not been validated and might have differing levels of quality ([#5](https://github.com/epage/parse-benchmarks-rs/issues/5))

# Running the Benchmarks

```bash
$ ./bench.py
$ ./format.py
```

[chumsky]: https://github.com/zesterer/chumsky
[chumsky-pratt]: https://docs.rs/chumsky/latest/chumsky/pratt/index.html
[combine]: https://github.com/Marwes/combine
[lalrpop]: https://github.com/lalrpop/lalrpop
[lelwel]: https://github.com/0x2a-42/lelwel
[logos]: https://github.com/maciejhirsz/logos
[nom]: https://github.com/geal/nom
[nom-pratt]: https://docs.rs/nom-language/latest/nom_language/precedence/fn.precedence.html
[parol]: https://github.com/jsinger67/parol
[peg]: https://github.com/kevinmehall/rust-peg
[pest]: https://github.com/pest-parser/pest
[pom]: https://github.com/j-f-liu/pom
[winnow]: https://github.com/winnow-rs/winnow
[yap]: https://github.com/jsdw/yap
[yap]: https://github.com/jsdw/yap
[grmtools]: https://crates.io/crates/cfgrammar
