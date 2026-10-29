# fast-float (Eisel-Lemire float parsing)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Data formats](../README.md) › [Numbers to and from text](README.md)</sub>

**Tier A** (reproducible public benchmark) · ❌ Doesn't hold up · Open source · `Rust` · `MIT OR Apache-2.0` · 🔴 abandoned

> Port of Lemire's fast_float that beat std's float parsing 2-8x on Rust 1.49; std absorbed the algorithm in 1.55 and the gap is now 1.0x-1.3x.

| | |
|---|---|
| **Beats** | std's str::parse::<f64> (FromStr) |
| **Claimed result** | README: faster than from_str by 2-8x; Rust 1.49 tables: canada 21.58 vs 174.43 ns (i7-4771), 25.90 vs 190.06 ns (AMD Rome). Current tables (fast-float2 README): canada 9.77 vs 11.59 ns (i7-14700K, Rust 1.81), 19.17 vs 22.90 ns (EPYC 7763, Rust 1.83); bi 57.52 vs 35.92 ns and 82.37 vs 62.20 ns (std faster) (ratios derived from page data) |
| **Workload** | Best ns per f64 parse on canada.txt, mesh.txt, uniform [0,1), big integer-valued floats (bi/iidi), %de%d and reciprocals of random u32. |
| **Metric** | latency |
| **Hardware** | Intel i7-4771 and AMD Rome (Rust 1.49 tables); Intel i7-14700K and AMD EPYC 7763 (current tables) |
| **OS** | macOS and Linux (1.49 tables); Ubuntu 24.04 WSL2 and Ubuntu 24.04.1 (current tables) |
| **Toolchain** | Rust 1.49 (original claim); Rust 1.81 and 1.83 (current comparison) |
| **Measured** | 2024-12 |
| **Adoption** | 292 stars |
| **Last commit** | 2021-07-03 |
| **Platforms** | portable; no_std |
| **Install** | cargo add fast-float2 (the maintained fork) |
| **Links** | [repository](https://github.com/aldanor/fast-float-rust) · [documentation](https://docs.rs/fast-float) · [benchmark](https://github.com/Alexhuszagh/fast-float-rust/blob/9d0bedb418eee56be31946ad318e629348dc0d43/README.md) |

## Verdict

The 2-8x lead over from_str was measured on Rust 1.49. Rust 1.55 merged a port of this crate's algorithm into std, and on Rust 1.81 and 1.83 fast-float is only 1.02x-1.29x faster on five datasets and 1.32x-1.60x slower than std on large integer-valued floats. The crate is abandoned with two RustSec advisories; fast-float2 is the patched fork for byte-slice and partial parsing.

**Why tier A:** Public benchmark tool (extras/simple-bench) with CPU, OS and Rust version stated for both the old and new tables.

## Use it when

- Never the original crate; use fast-float2 if you need parse_partial or parsing straight from &[u8]

## Skip it when

- Ordinary f64/f32 parsing: str::parse is within about 1.3x since Rust 1.55
- Any use of the unpatched fast-float crate: RUSTSEC-2024-0379 (unsound) and RUSTSEC-2025-0003 (segfault on empty input)

## How it is fast

Eisel-Lemire: for up to 19 significant digits it computes the result with one or two 64x64-bit multiplications against a table of powers of five, falling back to a slower exact path only for long inputs. On little-endian systems it has extra handling for numbers with more than eight digits after the decimal point.

## What it costs

- Unmaintained since 2021; unsafe code with known soundness bugs

## Problems with the evidence

- The old and new tables come from different CPUs; the ratios within each table are same-machine
- The current tables are in the fork's README, written by the author of std's implementation

## Evidence

<details>
<summary>15 quotes from the sources, checked 2026-10-28</summary>

> In particular, it is faster than Rust standard library's `FromStr::from_str()` by a factor of 2-8x
>
> — [github.com/aldanor/fast-float-rust/blob/cecc48926d64e97dbbe2fbc1a2e…](https://github.com/aldanor/fast-float-rust/blob/cecc48926d64e97dbbe2fbc1a2edc446941f2058/README.md)

> Intel i7-4771 3.5GHz, macOS, Rust 1.49.
>
> — [github.com/aldanor/fast-float-rust/blob/cecc48926d64e97dbbe2fbc1a2e…](https://github.com/aldanor/fast-float-rust/blob/cecc48926d64e97dbbe2fbc1a2edc446941f2058/README.md)

> | fast-float | 21.58 | 10.70 | 19.36 | 40.50 | 26.07 | 29.13 |
>
> — [github.com/aldanor/fast-float-rust/blob/cecc48926d64e97dbbe2fbc1a2e…](https://github.com/aldanor/fast-float-rust/blob/cecc48926d64e97dbbe2fbc1a2edc446941f2058/README.md)

> | from_str | 174.43 | 22.30 | 99.93 | 227.76 | 111.31 | 204.46 |
>
> — [github.com/aldanor/fast-float-rust/blob/cecc48926d64e97dbbe2fbc1a2e…](https://github.com/aldanor/fast-float-rust/blob/cecc48926d64e97dbbe2fbc1a2edc446941f2058/README.md)

> AMD Rome, Linux, Rust 1.49.
>
> — [github.com/aldanor/fast-float-rust/blob/cecc48926d64e97dbbe2fbc1a2e…](https://github.com/aldanor/fast-float-rust/blob/cecc48926d64e97dbbe2fbc1a2edc446941f2058/README.md)

> | fast-float | 25.90 | 12.12 | 20.54 | 47.01 | 29.23 | 32.36 |
>
> — [github.com/aldanor/fast-float-rust/blob/cecc48926d64e97dbbe2fbc1a2e…](https://github.com/aldanor/fast-float-rust/blob/cecc48926d64e97dbbe2fbc1a2edc446941f2058/README.md)

> | from_str | 190.06 | 26.10 | 102.44 | 239.87 | 119.04 | 211.73 |
>
> — [github.com/aldanor/fast-float-rust/blob/cecc48926d64e97dbbe2fbc1a2e…](https://github.com/aldanor/fast-float-rust/blob/cecc48926d64e97dbbe2fbc1a2edc446941f2058/README.md)

> - CPU: Intel i7-14700K 3.40GHz - OS: Ubuntu 24.04 (WSL2) - Rust: 1.81
>
> — [github.com/Alexhuszagh/fast-float-rust/blob/9d0bedb418eee56be31946a…](https://github.com/Alexhuszagh/fast-float-rust/blob/9d0bedb418eee56be31946ad318e629348dc0d43/README.md)

> | fast-float | 9.77 | 5.04 | 9.05 | 57.52 | 14.40 | 14.23 |
>
> — [github.com/Alexhuszagh/fast-float-rust/blob/9d0bedb418eee56be31946a…](https://github.com/Alexhuszagh/fast-float-rust/blob/9d0bedb418eee56be31946ad318e629348dc0d43/README.md)

> | from_str | 11.59 | 5.92 | 11.23 | 35.92 | 14.75 | 16.76 |
>
> — [github.com/Alexhuszagh/fast-float-rust/blob/9d0bedb418eee56be31946a…](https://github.com/Alexhuszagh/fast-float-rust/blob/9d0bedb418eee56be31946ad318e629348dc0d43/README.md)

> - CPU: AMD EPYC 7763 64-Core Processor 3.20GHz - OS: Ubuntu 24.04.1 - Rust: 1.83
>
> — [github.com/Alexhuszagh/fast-float-rust/blob/9d0bedb418eee56be31946a…](https://github.com/Alexhuszagh/fast-float-rust/blob/9d0bedb418eee56be31946ad318e629348dc0d43/README.md)

> | fast-float | 19.17 | 9.89 | 17.34 | 82.37 | 25.26 | 27.22 |
>
> — [github.com/Alexhuszagh/fast-float-rust/blob/9d0bedb418eee56be31946a…](https://github.com/Alexhuszagh/fast-float-rust/blob/9d0bedb418eee56be31946ad318e629348dc0d43/README.md)

> | from_str | 22.90 | 12.72 | 22.10 | 62.20 | 27.51 | 30.80 |
>
> — [github.com/Alexhuszagh/fast-float-rust/blob/9d0bedb418eee56be31946a…](https://github.com/Alexhuszagh/fast-float-rust/blob/9d0bedb418eee56be31946ad318e629348dc0d43/README.md)

> This implementation has been adopted by the Rust standard library
>
> — [github.com/Alexhuszagh/fast-float-rust/blob/9d0bedb418eee56be31946a…](https://github.com/Alexhuszagh/fast-float-rust/blob/9d0bedb418eee56be31946ad318e629348dc0d43/README.md)

> Segmentation fault due to lack of bound check
>
> — [github.com/rustsec/advisory-db/blob/main/crates/fast-float/RUSTSEC-…](https://github.com/rustsec/advisory-db/blob/main/crates/fast-float/RUSTSEC-2025-0003.md)

</details>
