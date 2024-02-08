# Rust Parsing Benchmarks

This repo tries to assess Rust parsing performance.

| crate     | parser type | action code | integration        | input type             | precedence climbing | parameterized rules | streaming input |
|-----------|-------------|-------------|--------------------|------------------------|---------------------|---------------------|-----------------|
| [chumsky] | combinators | in source   | library            | `&str`                 | ?                   | ?                   | ?               |
| [combine] | combinators | in source   | library            | `&str`                 | ?                   | ?                   | ?               |
| [lalrpop] | LR(1)       | in grammar  | build script       | `&str`                 | No                  | Yes                 | No              |
| [logos]   | lexer       | in source   | proc macro         | `&str`, `&[u8]`        | ?                   | ?                   | ?               |
| [nom]     | combinators | in source   | library            | `&[u8]`, custom        | No                  | Yes                 | Yes             |
| [pest]    | PEG         | external    | proc macro (file)  | `&str`                 | Yes                 | No                  | No              |
| [winnow]  | combinators | in source   | library            | `&str`, `&[T]`, custom | No                  | Yes                 | Yes             |
| [yap]     | combinators | in source   | library            | `&str`, `&[T]`, custom | No                  | Yes                 | ?               |

Formerly, we compared:
- [peg]: invalid example
- [pom]: lack of notoriety

# Results

Name | Overhead (release) | Build (debug) | Parse (release) | Downloads | Version
-----|--------------------|---------------|-----------------|-----------|--------
null | 0 KiB | 203ms | 4ms | - | -
chumsky | 662 KiB | 6s | 365ms | ![Download count](https://img.shields.io/crates/dr/ariadne) | v0.4.0
logos | 171 KiB | 5s | 17ms | ![Download count](https://img.shields.io/crates/dr/ariadne) | v0.4.0
combine | 184 KiB | 4s | 41ms | ![Download count](https://img.shields.io/crates/dr/combine) | v3.8.1
lalrpop | 1,619 KiB | 11s | 886ms | ![Download count](https://img.shields.io/crates/dr/lalrpop-util) | v0.20.0
nom | 100 KiB | 2s | 65ms | ![Download count](https://img.shields.io/crates/dr/nom) | v7.1.3
pest | 104 KiB | 4s | 51ms | ![Download count](https://img.shields.io/crates/dr/pest) | v2.7.6
serde_json | 42 KiB | 3s | 13ms | ![Download count](https://img.shields.io/crates/dr/serde_json) | v1.0.113
winnow | 74 KiB | 2s | 23ms | ![Download count](https://img.shields.io/crates/dr/winnow) | v0.5.36
yap | 59 KiB | 471ms | 31ms | ![Download count](https://img.shields.io/crates/dr/yap) | v0.12.0

*System: Linux 5.4.0-124-generic (x86_64), rustc 1.75.0 (82e1608df 2023-12-21) w/ `-j 8`*

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
