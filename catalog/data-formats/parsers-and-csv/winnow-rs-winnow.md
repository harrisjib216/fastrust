# winnow (parser combinators)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Data formats](../README.md) › [Parsers and CSV](README.md)</sub>

**Tier C** (first-party numbers) · 🟡 Holds up in part · Open source · `Rust` · `MIT` · 🟢 active

> Fork of nom that parsed canada.json 2.5x faster and built 3x faster than nom 8 in its author's comparison suite.

| | |
|---|---|
| **Beats** | nom 8.0.0 parser combinators |
| **Claimed result** | Parse (release) 27ms vs 68ms; debug build 1s vs 3s; binary overhead 67 KiB vs 88 KiB (winnow v1.0.1 vs nom v8.0.0); serde_json 18ms |
| **Workload** | Each library's example app parses nativejson-benchmark canada.json (number-heavy) once per process, timed with hyperfine; build time is a clean debug build; overhead is release binary size over a null app. |
| **Metric** | time |
| **Hardware** | x86_64 Linux machine, CPU model not stated |
| **OS** | Linux 6.17.9-76061709-generic (x86_64) |
| **Toolchain** | rustc 1.94.1 (e408947bf 2026-03-25), cargo -j 8 |
| **Measured** | 2026-04-13 |
| **Adoption** | 978 stars |
| **Last commit** | 2026-10-16 |
| **Platforms** | portable; no_std |
| **Install** | cargo add winnow |
| **Links** | [repository](https://github.com/winnow-rs/winnow) · [documentation](https://docs.rs/winnow) · [benchmark](https://github.com/rosetta-rs/parse-rosetta-rs/blob/fb5145752a7189ba2c85ec874e33ef0cbc9de658/README.md) |

## Verdict

parse-rosetta-rs shows winnow 1.0.1 parsing canada.json in 27ms vs 68ms for nom 8.0.0 and a 1s vs 3s debug build. But that is one number-heavy JSON file timed as whole-process wall clock, the README warns the parsers are not validated and differ in quality, the benchmark is maintained by winnow's author, and serde_json (18ms) beats both.

**Why tier C:** Public harness with kernel and rustc stated, but no CPU model; maintained by winnow's author.

## Use it when

- Writing new combinator parsers for text or binary formats where build time and parse speed both matter
- Migrating from nom; winnow keeps nom's model and ships a migration guide

## Skip it when

- Parsing JSON: use serde_json or a SIMD JSON crate, which beat both combinator libraries
- You depend on nom's ecosystem (nom-locate, nom-supreme) and do not want to port

## How it is fast

Parsers are Fn(&mut I) -> O rather than nom's pure Fn(I) -> (I, O), so Ok and Err values are smaller and more likely to stay in registers, and the default ContextError keeps error overhead low. winnow deliberately skips nom 8's GAT-based parser modes, so each parser is generated for one mode instead of up to eight, which cuts build time and binary size.

## What it costs

- API churn: winnow's README frames it as willing to change for future parsers, unlike nom's focus on stability

## Problems with the evidence

- Single workload (canada.json) and whole-process timing, including startup and file reading
- The README notes the parsers have not been validated and may differ in quality; the nom and winnow apps may use different float parsers
- Benchmark maintained by winnow's author; no CPU model

## Evidence

<details>
<summary>6 quotes from the sources, checked 2026-10-28</summary>

> nom | 88 KiB | 3s | 68ms | ![Download count](https://img.shields.io/crates/dr/nom) | v8.0.0
>
> — [github.com/rosetta-rs/parse-rosetta-rs/blob/fb5145752a7189ba2c85ec8…](https://github.com/rosetta-rs/parse-rosetta-rs/blob/fb5145752a7189ba2c85ec874e33ef0cbc9de658/README.md)

> serde_json | 57 KiB | 3s | 18ms | ![Download count](https://img.shields.io/crates/dr/serde_json) | v1.0.149
>
> — [github.com/rosetta-rs/parse-rosetta-rs/blob/fb5145752a7189ba2c85ec8…](https://github.com/rosetta-rs/parse-rosetta-rs/blob/fb5145752a7189ba2c85ec874e33ef0cbc9de658/README.md)

> winnow | 67 KiB | 1s | 27ms | ![Download count](https://img.shields.io/crates/dr/winnow) | v1.0.1
>
> — [github.com/rosetta-rs/parse-rosetta-rs/blob/fb5145752a7189ba2c85ec8…](https://github.com/rosetta-rs/parse-rosetta-rs/blob/fb5145752a7189ba2c85ec874e33ef0cbc9de658/README.md)

> *System: Linux 6.17.9-76061709-generic (x86_64), rustc 1.94.1 (e408947bf 2026-03-25) w/ `-j 8`*
>
> — [github.com/rosetta-rs/parse-rosetta-rs/blob/fb5145752a7189ba2c85ec8…](https://github.com/rosetta-rs/parse-rosetta-rs/blob/fb5145752a7189ba2c85ec874e33ef0cbc9de658/README.md)

> Parsers have not been validated and might have differing levels of quality
>
> — [github.com/rosetta-rs/parse-rosetta-rs/blob/fb5145752a7189ba2c85ec8…](https://github.com/rosetta-rs/parse-rosetta-rs/blob/fb5145752a7189ba2c85ec874e33ef0cbc9de658/README.md)

> json_path = pathlib.Path(__file__).parent / "third_party/nativejson-benchmark/data/canada.json"
>
> — [github.com/rosetta-rs/parse-rosetta-rs/blob/main/bench.py](https://github.com/rosetta-rs/parse-rosetta-rs/blob/main/bench.py)

</details>
