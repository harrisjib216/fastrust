# lexical-core (number parsing and formatting)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Data formats](../README.md) › [Numbers to and from text](README.md)</sub>

**Tier A** (reproducible public benchmark) · 🟡 Holds up in part · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Configurable number parsing and formatting; 2.8x faster than std at printing floats, but no longer the fastest and only 1.1x-1.5x ahead at parsing.

| | |
|---|---|
| **Beats** | std's str::parse::<f64> and Display for floats |
| **Claimed result** | f64 formatting 24.4 ns vs libcore 68.8 ns (2.82x) and 22.0 vs 63.9 ns (2.90x); zmij 8.5 ns, teju 21.4 ns. f64 parsing on i7-14700K, Rust 1.81: canada 10.62 vs 11.59 ns, bi 26.40 vs 35.92 ns; on EPYC 7763, Rust 1.83: mesh 8.41 vs 12.72 ns (1.09x-1.51x across datasets, derived from page data) |
| **Workload** | Formatting: random f64 at 1-17 significant digits (dtoa-benchmark). Parsing: ns per f64 on canada, mesh, uniform, big integer, %de%d and reciprocal datasets (fast-float2 README). |
| **Metric** | latency |
| **Hardware** | AMD Ryzen Threadripper 9975WX and Apple M4 Max (formatting); Intel i7-14700K and AMD EPYC 7763 (parsing) |
| **OS** | Ubuntu 24.04 (WSL2) and Ubuntu 24.04.1 for the parsing tables |
| **Toolchain** | Rust 1.97.0 with -C target-cpu=native (formatting); Rust 1.81 and 1.83 (parsing) |
| **Measured** | 2026-07-12 |
| **Adoption** | 363 stars |
| **Last commit** | 2026-03-12 |
| **Platforms** | portable; no_std without an allocator |
| **Install** | cargo add lexical-core |
| **Links** | [repository](https://github.com/Alexhuszagh/rust-lexical) · [documentation](https://docs.rs/lexical-core) · [benchmark](https://github.com/dtolnay/dtoa-benchmark/blob/a1838f938c130c8cf8d0453dc4109164a0c80a87/README.md) |

## Verdict

The README calls lexical the currently fastest Rust number formatter and parser, but dtoa-benchmark puts zmij (8.5 ns) and teju (21.4 ns) ahead of lexical (24.4 ns) for f64 formatting. For float parsing std has used Eisel-Lemire since 1.55 and trails lexical by only 1.09x-1.51x; the README itself points users to std for floats and to itoa for integers. Against std's float Display it is 2.8x-2.9x faster.

**Why tier A:** Public benchmarks with CPU and Rust version stated (dtoa-benchmark; fast-float2's simple-bench).

## Use it when

- Parsing number formats std rejects or treats differently (digit separators, other languages' syntax, JSON-strict rules) via its format options
- Partial parsers that return the number of bytes consumed
- Radix or f16/bf16 conversions

## Skip it when

- Plain f64 parsing: std's from_str is within 1.1x-1.5x since Rust 1.55
- Fastest float printing: zmij is 2.9x faster
- Integer formatting: the README itself suggests itoa

## How it is fast

Float parsing uses Eisel-Lemire with a slower exact fallback, the approach this author ported into std for Rust 1.55. Writing goes into a caller-provided byte buffer instead of through core::fmt::Formatter, and unused conversions are compiled out by feature flags.

## What it costs

- RUSTSEC-2023-0086 and RUSTSEC-2023-0055 (soundness) affected versions before lexical-core 1.0 and lexical 7.0
- Large feature surface and compile time unless features are trimmed

## Problems with the evidence

- Parsing numbers come from fast-float2's README, written by lexical's own author
- The 'Apple M1 (macOS)' table in fast-float2's README lists an AMD EPYC CPU, a copy-paste error, so that table is not used here
- lexical's README metrics are chart images

## Evidence

<details>
<summary>13 quotes from the sources, checked 2026-10-28</summary>

> Lexical is the currently fastest Rust number formatter and parser
>
> — [github.com/Alexhuszagh/rust-lexical](https://github.com/Alexhuszagh/rust-lexical)

> If you want a minimal, performant float parser, recent versions of the Rust standard library should be [sufficient](https://github.com/rust-lang/rust/pull/86761). For high-performance integer formatters, look at [itoa](https://docs.rs/itoa/latest/itoa/).
>
> — [github.com/Alexhuszagh/rust-lexical](https://github.com/Alexhuszagh/rust-lexical)

> The following results are measured on a 2025 AMD Ryzen Threadripper 9975WX and 2024 Apple M4 Max, each using Rust 1.97.0:
>
> — [github.com/dtolnay/dtoa-benchmark/blob/a1838f938c130c8cf8d0453dc410…](https://github.com/dtolnay/dtoa-benchmark/blob/a1838f938c130c8cf8d0453dc4109164a0c80a87/README.md)

> <tr><td><a href="https://doc.rust-lang.org/std/fmt/trait.Display.html">libcore</a></td><td align="right">68.8</td><td align="right">1.00&times;</td><td align="right">63.9</td><td align="right">1.00&times;</td></tr>
>
> — [github.com/dtolnay/dtoa-benchmark/blob/a1838f938c130c8cf8d0453dc410…](https://github.com/dtolnay/dtoa-benchmark/blob/a1838f938c130c8cf8d0453dc4109164a0c80a87/README.md)

> <tr><td><a href="https://github.com/Alexhuszagh/rust-lexical">lexical</a></td><td align="right">24.4</td><td align="right">2.82&times;</td><td align="right">22.0</td><td align="right">2.90&times;</td></tr>
>
> — [github.com/dtolnay/dtoa-benchmark/blob/a1838f938c130c8cf8d0453dc410…](https://github.com/dtolnay/dtoa-benchmark/blob/a1838f938c130c8cf8d0453dc4109164a0c80a87/README.md)

> <tr><td><a href="https://github.com/andrepd/teju-jagua-rs">teju</a></td><td align="right">21.4</td><td align="right">3.21&times;</td><td align="right">19.1</td><td align="right">3.35&times;</td></tr>
>
> — [github.com/dtolnay/dtoa-benchmark/blob/a1838f938c130c8cf8d0453dc410…](https://github.com/dtolnay/dtoa-benchmark/blob/a1838f938c130c8cf8d0453dc4109164a0c80a87/README.md)

> <tr><td><a href="https://github.com/dtolnay/zmij">zmij</a></td><td align="right">8.5</td><td align="right">8.09&times;</td><td align="right">5.9</td><td align="right">10.83&times;</td></tr>
>
> — [github.com/dtolnay/dtoa-benchmark/blob/a1838f938c130c8cf8d0453dc410…](https://github.com/dtolnay/dtoa-benchmark/blob/a1838f938c130c8cf8d0453dc4109164a0c80a87/README.md)

> - CPU: Intel i7-14700K 3.40GHz - OS: Ubuntu 24.04 (WSL2) - Rust: 1.81
>
> — [github.com/Alexhuszagh/fast-float-rust/blob/9d0bedb418eee56be31946a…](https://github.com/Alexhuszagh/fast-float-rust/blob/9d0bedb418eee56be31946ad318e629348dc0d43/README.md)

> | lexical | 10.62 | 4.93 | 9.92 | 26.40 | 12.43 | 14.40 |
>
> — [github.com/Alexhuszagh/fast-float-rust/blob/9d0bedb418eee56be31946a…](https://github.com/Alexhuszagh/fast-float-rust/blob/9d0bedb418eee56be31946ad318e629348dc0d43/README.md)

> | from_str | 11.59 | 5.92 | 11.23 | 35.92 | 14.75 | 16.76 |
>
> — [github.com/Alexhuszagh/fast-float-rust/blob/9d0bedb418eee56be31946a…](https://github.com/Alexhuszagh/fast-float-rust/blob/9d0bedb418eee56be31946ad318e629348dc0d43/README.md)

> - CPU: AMD EPYC 7763 64-Core Processor 3.20GHz - OS: Ubuntu 24.04.1 - Rust: 1.83
>
> — [github.com/Alexhuszagh/fast-float-rust/blob/9d0bedb418eee56be31946a…](https://github.com/Alexhuszagh/fast-float-rust/blob/9d0bedb418eee56be31946ad318e629348dc0d43/README.md)

> | lexical | 18.89 | 8.41 | 16.83 | 47.66 | 22.08 | 26.99 |
>
> — [github.com/Alexhuszagh/fast-float-rust/blob/9d0bedb418eee56be31946a…](https://github.com/Alexhuszagh/fast-float-rust/blob/9d0bedb418eee56be31946ad318e629348dc0d43/README.md)

> | from_str | 22.90 | 12.72 | 22.10 | 62.20 | 27.51 | 30.80 |
>
> — [github.com/Alexhuszagh/fast-float-rust/blob/9d0bedb418eee56be31946a…](https://github.com/Alexhuszagh/fast-float-rust/blob/9d0bedb418eee56be31946ad318e629348dc0d43/README.md)

</details>
