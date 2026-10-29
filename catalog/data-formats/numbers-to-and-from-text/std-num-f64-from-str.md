# std float parsing (Eisel-Lemire, Rust 1.55)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Data formats](../README.md) › [Numbers to and from text](README.md)</sub>

**Tier A** (reproducible public benchmark) · ✅ Holds up · Ships with Rust · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Since Rust 1.55, str::parse::<f64> uses Eisel-Lemire and runs within about 1.2x of fast-float, down from 7x-8x behind.

| | |
|---|---|
| **Beats** | std's float parsing before Rust 1.55 (dec2flt with Clinger's AlgorithmM and Bellerophon) |
| **Claimed result** | PR #86761: old std 130 MiB/s on canada vs up to 1200 MiB/s for Eisel-Lemire on Apple M1 (10x). canada from_str vs fast-float: 174.43 vs 21.58 ns (Rust 1.49, i7-4771) and 190.06 vs 25.90 ns (Rust 1.49, AMD Rome); 11.59 vs 9.77 ns (Rust 1.81, i7-14700K) and 22.90 vs 19.17 ns (Rust 1.83, EPYC 7763) (ratios derived from page data) |
| **Workload** | Parsing f64 from text: canada.txt coordinates and synthetic datasets; ns per value or MiB/s. |
| **Metric** | latency |
| **Hardware** | Apple M1 (PR figure); Intel i7-4771 and AMD Rome (Rust 1.49 tables); Intel i7-14700K and AMD EPYC 7763 (current tables) |
| **OS** | macOS and Linux |
| **Toolchain** | Rust 1.49 (before) vs Rust 1.81 and 1.83 (after) |
| **Measured** | 2024-12 |
| **Platforms** | all targets (core) |
| **Install** | built into std since 1.55 |
| **Links** | [documentation](https://blog.rust-lang.org/2021/09/09/Rust-1.55.0/) · [benchmark](https://github.com/rust-lang/rust/pull/86761) |

## Verdict

Rust 1.55 (2021-09-09) replaced dec2flt with an Eisel-Lemire implementation adapted from fast-float. The PR put the old parser at 130 MiB/s on canada against up to 1200 MiB/s for the new algorithm, and published tables show from_str going from 7.3x-8.1x behind fast-float on Rust 1.49 to 1.19x-1.20x behind on Rust 1.81/1.83, and ahead on large integer-valued floats.

**Why tier A:** The fast-float tables come from a public benchmark tool with CPU, OS and Rust version stated; the throughput figures are from the rust-lang PR.

## Use it when

- Any f32/f64 parsing from &str: this is the default and needs no dependency

## Skip it when

- You need partial parsing or parsing straight from &[u8]; use fast-float2 or lexical-core

## How it is fast

Eisel-Lemire handles inputs with up to 19 significant digits using 128-bit products against a power-of-five table, with a fast path for exactly representable values and a simple big-decimal fallback for long inputs. The PR also parses eight digits at a time with three multiplications.

## What it costs

- None for callers; the PR reports similar stripped code size and much smaller unstripped binaries

## Problems with the evidence

- Before/after ratios compare tables from different CPUs; each ratio is within one machine
- The 1200 MiB/s figure is for the algorithm on Apple M1, not a measurement of the merged std code

## Evidence

<details>
<summary>16 quotes from the sources, checked 2026-10-28</summary>

> The standard library's implementation of float parsing has been updated to use the Eisel-Lemire algorithm, which brings both speed improvements and improved correctness.
>
> — [blog.rust-lang.org/2021/09/09/Rust-1.55.0/](https://blog.rust-lang.org/2021/09/09/Rust-1.55.0/)

> - [Updated std's float parsing to use the Eisel-Lemire algorithm.][86761]
>
> — [github.com/rust-lang/rust/blob/master/RELEASES.md](https://github.com/rust-lang/rust/blob/master/RELEASES.md)

> speeds up to 1200 MiB/s on Apple's M1 architecture
>
> — [api.github.com/repos/rust-lang/rust/pulls/86761](https://api.github.com/repos/rust-lang/rust/pulls/86761)

> 10x faster than Rust's 130 MiB/s.
>
> — [api.github.com/repos/rust-lang/rust/pulls/86761](https://api.github.com/repos/rust-lang/rust/pulls/86761)

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

</details>
