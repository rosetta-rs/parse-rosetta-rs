# Rust Parsing Benchmarks

This repo tries to assess Rust parsing performance.

| crate      | parser type   | action code | integration        | input type              | precedence             | parameterized rules | streaming input |
|------------|---------------|-------------|--------------------|-------------------------|------------------------|---------------------|-----------------|
| [chumsky]  | combinators   | in source   | library            | `&str`, `&[u8]`, custom | [pratt][chumsky-pratt] | Yes                 | Yes             |
| [combine]  | combinators   | in source   | library            | `&str`                  | ?                      | ?                   | ?               |
| [grmtools] | CFG           | in grammar  | library            | ?                       | ?                      | ?                   | ?               |
| [lalrpop]  | LR(1)         | in grammar  | build script       | `&str`                  | none                   | Yes                 | No              |
| [logos]    | lexer         | in source   | proc macro         | `&str`, `&[u8]`         | ?                      | ?                   | ?               |
| [nom]      | combinators   | in source   | library            | `&str`, `&[u8]`, custom | [pratt][nom-pratt]     | Yes                 | Yes             |
| [parol]    | LL(k)/LALR(1) | in source   | build script       | `&str`                  | climbing               | No                  | No              |
| [peg]      | PEG           | in grammar  | proc macro (block) | `&str`, `&[T]`, custom  | climbing               | Yes                 | No              |
| [pest]     | PEG           | external    | proc macro (file)  | `&str`                  | climbing               | No                  | No              |
| [winnow]   | combinators   | in source   | library            | `&str`, `&[T]`, custom  | none                   | Yes                 | Yes             |
| [yap]      | combinators   | in source   | library            | `&str`, `&[T]`, custom  | none                   | Yes                 | ?               |

Formerly, we compared:
- [pom]: lack of notoriety
- [lelwel]: example is too different than others

# Results

Name | Overhead (release) | Build (debug) | Parse (release) | Downloads | Version
-----|--------------------|---------------|-----------------|-----------|--------
null | 0 KiB | 249ms | 3ms | - | -
grmtools | 2,610 KiB | 11s | 164ms | ![Download count](https://img.shields.io/crates/dr/cfgrammar) | v0.13.10
chumsky | 150 KiB | 4s | 32ms | ![Download count](https://img.shields.io/crates/dr/chumsky) | v0.10.1
combine | 181 KiB | 4s | 53ms | ![Download count](https://img.shields.io/crates/dr/combine) | v3.8.1
lalrpop | 1,523 KiB | 11s | 37ms | ![Download count](https://img.shields.io/crates/dr/lalrpop) | v0.22.2
logos | 90 KiB | 4s | 21ms | ![Download count](https://img.shields.io/crates/dr/logos) | v0.15.0
nom | 98 KiB | 3s | 65ms | ![Download count](https://img.shields.io/crates/dr/nom) | v8.0.0
parol | 492 KiB | 9s | 174ms | ![Download count](https://img.shields.io/crates/dr/parol) | v4.1.0
peg | 80 KiB | 2s | 23ms | ![Download count](https://img.shields.io/crates/dr/peg) | v0.8.5
pest | 130 KiB | 5s | 61ms | ![Download count](https://img.shields.io/crates/dr/pest) | v2.8.1
serde_json | 59 KiB | 3s | 14ms | ![Download count](https://img.shields.io/crates/dr/serde_json) | v1.0.142
winnow | 75 KiB | 2s | 28ms | ![Download count](https://img.shields.io/crates/dr/winnow) | v0.7.12
yap | 61 KiB | 527ms | 33ms | ![Download count](https://img.shields.io/crates/dr/yap) | v0.12.0

*System: Linux 6.8.0-62-generic (x86_64), rustc 1.89.0 (29483883e 2025-08-04) w/ `-j 8`*

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
