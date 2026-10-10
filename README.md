# Rust Parsing Benchmarks

This repo tries to assess Rust parsing performance.

| crate      | parser type   | action code | integration        | input type              |
|------------|---------------|-------------|--------------------|-------------------------|
| [chumsky]  | combinators   | in source   | library            | `&str`, `&[u8]`, custom |
| [combine]  | combinators   | in source   | library            | `&str`                  |
| [grmtools] | CFG           | in grammar  | library            | ?                       |
| [lalrpop]  | LR(1)         | in grammar  | build script       | `&str`                  |
| [lelwel]   | LL(1)         | in source   | build script       | `&str`                  |
| [logos]    | lexer         | in source   | proc macro         | `&str`, `&[u8]`         |
| [nom]      | combinators   | in source   | library            | `&str`, `&[u8]`, custom |
| [parol]    | LL(k)/LALR(1) | in source   | build script       | `&str`                  |
| [peg]      | PEG           | in grammar  | proc macro (block) | `&str`, `&[T]`, custom  |
| [pest]     | PEG           | external    | proc macro (file)  | `&str`                  |
| [winnow]   | combinators   | in source   | library            | `&str`, `&[T]`, custom  |
| [yap]      | combinators   | in source   | library            | `&str`, `&[T]`, custom  |

Formerly, we compared:
- [pom]: lack of notoriety

# Features

| crate    | operator precedence    | parameterized rules | streaming input | lossless syntax tree
|----------|------------------------|---------------------|-----------------|---------------------
| chumsky  | [pratt][chumsky-pratt] | ✅                  | ✅              | ❌
| combine  | ?                      | ?                   | ?               | ❌
| grmtools | ?                      | ?                   | ?               | ❌
| lalrpop  | ❌                     | ✅                  | ❌              | ❌
| lelwel   | [pratt][lelwel-pratt]  | ❌                  | ❌              | ✅
| logos    | ❌                     | ❌                  | ?               | ❌
| nom      | [pratt][nom-pratt]     | ✅                  | ✅              | ❌
| parol    | ❌                     | ❌                  | ❌              | ✅
| peg      | climbing               | ✅                  | ❌              | ❌
| pest     | climbing               | ❌                  | ❌              | ❌
| winnow   | ❌                     | ✅                  | ✅              | ❌
| yap      | ❌                     | ✅                  | ?               | ❌

# Results

Name | Overhead (release) | Build (debug) | Parse (release) | Downloads | Version
-----|--------------------|---------------|-----------------|-----------|--------
null | 0 KiB | 164ms | 2ms | - | -
grmtools | 2,612 KiB | 10s | 182ms | ![Download count](https://img.shields.io/crates/dr/cfgrammar) | v0.14.1
chumsky | 161 KiB | 5s | 47ms | ![Download count](https://img.shields.io/crates/dr/chumsky) | v0.12.0
combine | 175 KiB | 4s | 53ms | ![Download count](https://img.shields.io/crates/dr/combine) | v3.8.1
lalrpop | 1,523 KiB | 12s | 40ms | ![Download count](https://img.shields.io/crates/dr/lalrpop) | v0.23.1
lelwel | 175 KiB | 7s | 33ms | ![Download count](https://img.shields.io/crates/dr/lelwel) | v0.10.4
logos | 71 KiB | 6s | 22ms | ![Download count](https://img.shields.io/crates/dr/logos) | v0.16.1
nom | 88 KiB | 3s | 68ms | ![Download count](https://img.shields.io/crates/dr/nom) | v8.0.0
parol | 480 KiB | 9s | 184ms | ![Download count](https://img.shields.io/crates/dr/parol) | v4.4.0
peg | 77 KiB | 2s | 29ms | ![Download count](https://img.shields.io/crates/dr/peg) | v0.8.5
pest | 119 KiB | 3s | 62ms | ![Download count](https://img.shields.io/crates/dr/pest) | v2.8.6
serde_json | 57 KiB | 3s | 18ms | ![Download count](https://img.shields.io/crates/dr/serde_json) | v1.0.149
winnow | 67 KiB | 1s | 27ms | ![Download count](https://img.shields.io/crates/dr/winnow) | v1.0.1
yap | 60 KiB | 420ms | 40ms | ![Download count](https://img.shields.io/crates/dr/yap) | v0.12.0

*System: Linux 6.17.9-76061709-generic (x86_64), rustc 1.94.1 (e408947bf 2026-03-25) w/ `-j 8`*

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
[lelwel-pratt]: https://github.com/0x2a-42/lelwel?tab=readme-ov-file#direct-left-recursion
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
