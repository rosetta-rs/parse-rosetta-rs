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
null | 0 KiB | 408ms | 29ms | - | -
chumsky | 887 KiB | 6s | 799ms | ![Download count](https://img.shields.io/crates/dr/ariadne) | v0.1.5
nom | 160 KiB | 2s | 513ms | ![Download count](https://img.shields.io/crates/dr/nom) | v7.1.1

*System: Linux 5.10.16.3-microsoft-standard-WSL2 (x86_64) w/ `-j 20`*

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
