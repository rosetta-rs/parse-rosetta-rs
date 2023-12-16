# Rust Parsing Benchmarks

This repo tries to assess Rust parsing performance.

| crate     | parser type | action code | integration        | input type              | precedence climbing | parameterized rules | streaming input                                                         |
|-----------|-------------|-------------|--------------------|-------------------------|---------------------|---------------------|-------------------------------------------------------------------------|
| [chumsky] | combinators | in source   | library            | `&str`, `&[T]`, custom  | ?                   | ?                   | [Yes](https://docs.rs/chumsky/latest/chumsky/stream/struct.Stream.html) |
| [combine] | combinators | in source   | library            | `&str`, `&[T]`, custom  | ?                   | ?                   | [Yes](https://docs.rs/combine/latest/combine/stream/index.html)         |
| [lalrpop] | LR(1)       | in grammar  | build script       | `&str`                  | No                  | Yes                 | No                                                                      |
| [nom]     | combinators | in source   | library            | `&str`, `&[u8]`, custom | No                  | Yes                 | [Yes](https://docs.rs/nom/latest/nom/bytes/streaming/index.html)        |
| [peg]     | PEG         | in grammar  | proc macro (block) | `&str`, `&[T]`, custom  | Yes                 | Yes                 | No                                                                      |
| [pest]    | PEG         | external    | proc macro (file)  | `&str`                  | Yes                 | No                  | No                                                                      |
| [pom]     | combinators | in source   | library            | `&str`                  | ?                   | ?                   | No                                                                      |
| [winnow]  | combinators | in source   | library            | `&str`, `&[T]`, custom  | No                  | Yes                 | [Yes](https://docs.rs/winnow/latest/winnow/stream/index.html)           |
| [yap]     | combinators | in source   | library            | `&str`, `&[T]`, custom  | No                  | Yes                 | [Yes](https://docs.rs/yap_streaming/)                                   |

# Results

Name | Overhead (release) | Build (debug) | Parse (release) | Downloads | Version
-----|--------------------|---------------|-----------------|-----------|--------
null | 0 KiB | 227ms | 24ms | - | -
chumsky | 672 KiB | 7s | 1s | ![Download count](https://img.shields.io/crates/dr/ariadne) | v0.3.0
combine | 223 KiB | 4s | 880ms | ![Download count](https://img.shields.io/crates/dr/combine) | v3.8.1
lalrpop | 1,607 KiB | 12s | 2s | ![Download count](https://img.shields.io/crates/dr/lalrpop-util) | v0.20.0
nom | 141 KiB | 2s | 932ms | ![Download count](https://img.shields.io/crates/dr/nom) | v7.1.3
peg | 27 KiB | 2s | **invalid** | ![Download count](https://img.shields.io/crates/dr/peg) | v0.8.1
pest | 157 KiB | 4s | 844ms | ![Download count](https://img.shields.io/crates/dr/pest) | v2.7.3
pom | 187 KiB | 2s | 2s | ![Download count](https://img.shields.io/crates/dr/pom) | v3.3.0
winnow | 121 KiB | 2s | 729ms | ![Download count](https://img.shields.io/crates/dr/winnow) | v0.5.15
yap | 89 KiB | 479ms | 765ms | ![Download count](https://img.shields.io/crates/dr/yap) | v0.10.0

*System: Linux 5.4.0-124-generic (x86_64) w/ `-j 8`*

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
[yap]: https://github.com/jsdw/yap
