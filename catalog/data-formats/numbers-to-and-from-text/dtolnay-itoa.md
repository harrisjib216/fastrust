# itoa (integer-to-string)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Data formats](../README.md) › [Numbers to and from text](README.md)</sub>

**Tier D** (directional only) · 🟡 Holds up in part · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Integer-to-decimal formatting without core::fmt::Formatter; std 1.98's NumBuffer::format_into now offers the same path built in.

| | |
|---|---|
| **Beats** | std's Display for integers (format!, to_string, write!) |
| **Claimed result** | No numbers in text. itoa-benchmark shows a chart only, measured on a 2025 AMD Ryzen Threadripper 9975WX using Rust 1.92.0. |
| **Workload** | Random u32, u64 and u128 values with equal counts per type and digit length; fastest pass per group. |
| **Metric** | latency |
| **Hardware** | AMD Ryzen Threadripper 9975WX |
| **Toolchain** | Rust 1.92.0, --release, -C target-cpu=native (repo .cargo/config.toml) |
| **Measured** | 2026-01-17 |
| **Adoption** | 383 stars |
| **Last commit** | 2026-06-24 |
| **Platforms** | portable; no_std |
| **Install** | cargo add itoa |
| **Links** | [repository](https://github.com/dtolnay/itoa) · [documentation](https://docs.rs/itoa) · [benchmark](https://github.com/dtolnay/itoa-benchmark/blob/92abed15c76763657fec7d226da87d91a3caee1d/README.md) |

## Verdict

itoa publishes its results only as a chart, so the size of its lead cannot be quoted. Since Rust 1.98 (2026-08-20) std has core::fmt::NumBuffer and integer format_into, which call the same internal digit routine (_fmt_inner) straight into a stack buffer and skip Formatter, the overhead itoa exists to avoid. No published benchmark compares itoa with format_into.

**Why tier D:** Numbers exist only in a chart image (itoa README and itoa-benchmark).

## Use it when

- Toolchains older than 1.98, where std has no Formatter-free integer API
- Libraries with an MSRV below 1.98 (itoa's MSRV is 1.68)

## Skip it when

- Rust 1.98 or later: try core::fmt::NumBuffer with {integer}::format_into first
- Code that formats a handful of integers, where Formatter overhead does not matter

## How it is fast

Uses libcore's own two-digits-at-a-time lookup-table routine but writes into a fixed stack Buffer and returns &str, skipping the Formatter machinery (padding, flags, dynamic dispatch through fmt::Write).

## What it costs

- Another dependency for something std now covers on 1.98+

## Problems with the evidence

- Chart-only results; no text numbers to quote
- Benchmark run on Rust 1.92.0, before NumBuffer was stable; std's format_into was not measured
- Benchmark is maintained by itoa's author and builds with -C target-cpu=native

## Evidence

<details>
<summary>5 quotes from the sources, checked 2026-10-28</summary>

> The implementation comes straight from [libcore] but avoids the performance penalty of going through [`core::fmt::Formatter`].
>
> — [github.com/dtolnay/itoa](https://github.com/dtolnay/itoa)

> The following results are measured on a 2025 AMD Ryzen Threadripper 9975WX using Rust 1.92.0.
>
> — [github.com/dtolnay/itoa-benchmark/blob/92abed15c76763657fec7d226da8…](https://github.com/dtolnay/itoa-benchmark/blob/92abed15c76763657fec7d226da87d91a3caee1d/README.md)

> #[stable(feature = "int_format_into", since = "1.98.0")]
>
> — [github.com/rust-lang/rust/blob/1.99.0/library/core/src/fmt/num.rs](https://github.com/rust-lang/rust/blob/1.99.0/library/core/src/fmt/num.rs)

> offset = self._fmt_inner(&mut buf.buf);
>
> — [github.com/rust-lang/rust/blob/1.99.0/library/core/src/fmt/num.rs](https://github.com/rust-lang/rust/blob/1.99.0/library/core/src/fmt/num.rs)

> A buffer wrapper of which the internal size is based on the maximum number of digits the associated integer can have.
>
> — [doc.rust-lang.org/stable/core/fmt/struct.NumBuffer.html](https://doc.rust-lang.org/stable/core/fmt/struct.NumBuffer.html)

</details>
