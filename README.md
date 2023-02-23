# Rust Parsing Benchmarks

This repo tries to assess Rust parsing performance.

| crate     | parser type | action code | integration        | input type             | precedence climbing | parameterized rules | streaming input |
|-----------|-------------|-------------|--------------------|------------------------|---------------------|---------------------|-----------------|
| [chumsky] | combinators | in source   | library            | `&str`                 | ?                   | ?                   | ?               |
| [combine] | combiantors | in source   | library            | `&str`                 | ?                   | ?                   | ?               |
| [lalrpop] | LR(1)       | in grammar  | build script       | `&str`                 | No                  | Yes                 | No              |
| [nom]     | combinators | in source   | library            | `&[u8]`, custom        | No                  | Yes                 | Yes             |
| [peg]     | PEG         | in grammar  | proc macro (block) | `&str`, `&[T]`, custom | Yes                 | Yes                 | No              |
| [pest]    | PEG         | external    | proc macro (file)  | `&str`                 | Yes                 | No                  | No              |
| [pom]     | combiantors | in source   | library            | `&str`                 | ?                   | ?                   | ?               |
| [winnow]  | combinators | in source   | library            | `&str`, `&[T]`, custom  | No                 | Yes                 | Yes             |

# Results

Name | Overhead (release) | Build (debug) | Parse (release) | Downloads | Version
-----|--------------------|---------------|-----------------|-----------|--------
null | 0 KiB | 391ms | 22ms | - | -
chumsky | 773 KiB | 8s | 1s | ![Download count](https://img.shields.io/crates/dr/ariadne) | v0.1.5
combine | 228 KiB | 5s | 867ms | ![Download count](https://img.shields.io/crates/dr/combine) | v3.8.1
nom | 161 KiB | 2s | 848ms | ![Download count](https://img.shields.io/crates/dr/nom) | v7.1.3
peg | 29 KiB | 2s | **invalid** | ![Download count](https://img.shields.io/crates/dr/peg) | v0.8.1
pest | 143 KiB | 5s | 757ms | ![Download count](https://img.shields.io/crates/dr/pest) | v2.5.5
pom | 215 KiB | 890ms | 2s | ![Download count](https://img.shields.io/crates/dr/pom) | v3.2.0
winnow | 143 KiB | 2s | 747ms | ![Download count](https://img.shields.io/crates/dr/winnow) | v0.3.0

*System: Linux 5.4.0-104-generic (x86_64) w/ `-j 8`*

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
[nom]: https://github.com/geal/nom
[peg]: https://github.com/kevinmehall/rust-peg
[pest]: https://github.com/pest-parser/pest
[pom]: https://github.com/j-f-liu/pom
[winnow]: https://github.com/winnow-rs/winnow
