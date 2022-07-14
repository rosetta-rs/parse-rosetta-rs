# Rust Markdown Parsing Benchmarks

This repo tries to assess Rust parsing performance.

| crate     | parser type | action code | integration        | input type             | precedence climbing | parameterized rules | streaming input |
|-----------|-------------|-------------|--------------------|------------------------|---------------------|---------------------|-----------------|
| [peg]     | PEG         | in grammar  | proc macro (block) | `&str`, `&[T]`, custom | Yes                 | Yes                 | No              |
| [pest]    | PEG         | external    | proc macro (file)  | `&str`                 | Yes                 | No                  | No              |
| [nom]     | combinators | in source   | library            | `&[u8]`, custom        | No                  | Yes                 | Yes             |
| [chumsky] | combinators | in source   | library            | `&str`                 | ?                   | ?                   | ?               |
| [combine] | combiantors | in source   | library            | `&str`                 | ?                   | ?                   | ?               |
| [lalrpop] | LR(1)       | in grammar  | build script       | `&str`                 | No                  | Yes                 | No              |

# Results

Name | Overhead (release) | Build (debug) | Parse (release) | Downloads | Version
-----|--------------------|---------------|-----------------|-----------|--------
null | 0 KiB | 276ms | 28ms | - | -
chumsky | 887 KiB | 5s | 740ms | ![Download count](https://img.shields.io/crates/dr/ariadne) | v0.1.5
nom | 160 KiB | 2s | 504ms | ![Download count](https://img.shields.io/crates/dr/nom) | v7.1.1
pom | 217 KiB | 603ms | 848ms | ![Download count](https://img.shields.io/crates/dr/pom) | v3.2.0

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
