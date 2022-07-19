# Rust Parsing Benchmarks

This repo tries to assess Rust parsing performance.

| crate     | parser type | action code | integration        | input type               | precedence climbing | parameterized rules | streaming input |
|-----------|-------------|-------------|--------------------|------------------------|---------------------|---------------------|-----------------|
| [chumsky] | combinators | in source   | library            | `&str`                 | ?                   | ?                   | ?               |
| [combine] | combiantors | in source   | library            | `&str`                 | ?                   | ?                   | ?               |
| [lalrpop] | LR(1)       | in grammar  | build script       | `&str`                 | No                  | Yes                 | No              |
| [nom]     | combinators | in source   | library            | `&[u8]`, custom        | No                  | Yes                 | Yes             |
| [peg]     | PEG         | in grammar  | proc macro (block) | `&str`, `&[T]`, custom | Yes                 | Yes                 | No              |
| [pest]    | PEG         | external    | proc macro (file)  | `&str`                 | Yes                 | No                  | No              |
| [pom]     | combiantors | in source   | library            | `&str`                 | ?                   | ?                   | ?               |

# Results

Name | Overhead (release) | Build (debug) | Parse (release) | Downloads | Version
-----|--------------------|---------------|-----------------|-----------|--------
null | 0 KiB | 306ms | 28ms | - | -
chumsky | 887 KiB | 5s | 730ms | ![Download count](https://img.shields.io/crates/dr/ariadne) | v0.1.5
combine | 214 KiB | 3s | 452ms | ![Download count](https://img.shields.io/crates/dr/combine) | v3.8.1
nom | 160 KiB | 2s | 507ms | ![Download count](https://img.shields.io/crates/dr/nom) | v7.1.1
peg | 33 KiB | 2s | **invalid** | ![Download count](https://img.shields.io/crates/dr/peg) | v0.8.0
pest | 138 KiB | 940ms | 416ms | ![Download count](https://img.shields.io/crates/dr/pest) | v2.1.3
pom | 217 KiB | 628ms | 816ms | ![Download count](https://img.shields.io/crates/dr/pom) | v3.2.0

*System: Linux 5.10.16.3-microsoft-standard-WSL2 (x86_64) w/ `-j 20`*

Note:
- For more "Parse (release)" comparisons, see [parser_benchmarks](https://github.com/rust-bakery/parser_benchmarks)
- Parsers have not been validated and might have differing levels of quality ([#5](https://github.com/epage/parse-benchmarks-rs/issues/5))

# Running the Benchmarks

```bash
$ ./bench.py
$ ./format.py
```

[peg]: https://github.com/kevinmehall/rust-peg
[pest]: https://github.com/pest-parser/pest
[nom]: https://github.com/geal/nom
[lalrpop]: https://github.com/lalrpop/lalrpop
[chumsky]: https://github.com/zesterer/chumsky
[combine]: https://github.com/Marwes/combine
[pom]: https://github.com/j-f-liu/pom
