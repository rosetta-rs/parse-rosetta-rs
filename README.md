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
null | 0 KiB | 190ms | 2ms | - | -
grmtools | 2,783 KiB | 10s | 187ms | ![Download count](https://img.shields.io/crates/dr/cfgrammar) | v0.14.1
chumsky | 161 KiB | 4s | 46ms | ![Download count](https://img.shields.io/crates/dr/chumsky) | v0.12.0
combine | 175 KiB | 4s | 52ms | ![Download count](https://img.shields.io/crates/dr/combine) | v3.8.1
lalrpop | 1,523 KiB | 12s | 40ms | ![Download count](https://img.shields.io/crates/dr/lalrpop) | v0.23.0
logos | 71 KiB | 6s | 22ms | ![Download count](https://img.shields.io/crates/dr/logos) | v0.16.1
nom | 88 KiB | 3s | 67ms | ![Download count](https://img.shields.io/crates/dr/nom) | v8.0.0
parol | 480 KiB | 8s | 185ms | ![Download count](https://img.shields.io/crates/dr/parol) | v4.3.0
peg | 77 KiB | 2s | 28ms | ![Download count](https://img.shields.io/crates/dr/peg) | v0.8.5
pest | 119 KiB | 3s | 59ms | ![Download count](https://img.shields.io/crates/dr/pest) | v2.8.6
serde_json | 57 KiB | 3s | 18ms | ![Download count](https://img.shields.io/crates/dr/serde_json) | v1.0.149
winnow | 67 KiB | 2s | 29ms | ![Download count](https://img.shields.io/crates/dr/winnow) | v0.7.14
yap | 60 KiB | 478ms | 39ms | ![Download count](https://img.shields.io/crates/dr/yap) | v0.12.0

*System: Linux 6.17.9-76061709-generic (x86_64), rustc 1.94.0 (4a4ef493e 2026-03-02) w/ `-j 8`*

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
