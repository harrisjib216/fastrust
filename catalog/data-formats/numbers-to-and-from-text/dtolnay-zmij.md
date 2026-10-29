# zmij (Żmij float-to-string)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Data formats](../README.md) › [Numbers to and from text](README.md)</sub>

**Tier A** (reproducible public benchmark) · ✅ Holds up · Open source · `Rust` · `MIT` · 🟢 active

> Rust port of Victor Zverovich's Żmij float printer: 8x-11x faster than std's float Display and about 4x faster than ryu.

| | |
|---|---|
| **Beats** | std's Display for f64 (format!, to_string, write!) |
| **Claimed result** | 8.5 ns vs 68.8 ns per f64 (8.09x) on Threadripper 9975WX; 5.9 ns vs 63.9 ns (10.83x) on Apple M4 Max; vs ryu 31.1 ns and 26.9 ns (3.7x and 4.6x, derived from page data) |
| **Workload** | Random f64 bit patterns truncated to 1-17 significant digits, equal counts per precision; fastest of several passes per precision group, averaged over the 17 groups; one format_finite call per value. |
| **Metric** | latency |
| **Hardware** | AMD Ryzen Threadripper 9975WX; Apple M4 Max |
| **Toolchain** | Rust 1.97.0, --release, -C target-cpu=native (repo .cargo/config.toml) |
| **Measured** | 2026-07-12 |
| **Adoption** | 201 stars |
| **Last commit** | 2026-08-06 |
| **Platforms** | portable; no_std |
| **Install** | cargo add zmij |
| **Links** | [repository](https://github.com/dtolnay/zmij) · [documentation](https://docs.rs/zmij) · [benchmark](https://github.com/dtolnay/dtoa-benchmark/blob/a1838f938c130c8cf8d0453dc4109164a0c80a87/README.md) |

## Verdict

dtoa-benchmark on Rust 1.97.0 measures 8.5 ns per f64 against 68.8 ns for libcore on a Threadripper 9975WX (8.09x) and 5.9 vs 63.9 ns on an M4 Max (10.83x), and 3.7x-4.6x faster than ryu. serde_json switched its float output from ryu to zmij in 1.0.147, so serde_json users already get it.

**Why tier A:** Public benchmark (cargo run --release) with CPUs, Rust 1.97.0 and -C target-cpu=native (repo config) stated; written by zmij's porter.

## Use it when

- Writing many floats as text: CSV, JSON, logs, metrics exporters
- Replacing ryu, which it beats by about 3.7x on x86_64 in the same benchmark

## Skip it when

- You need std's exact Display output (std never switches to exponent notation); check the output format first
- You only print a few floats; the absolute savings are tens of nanoseconds each
- You use serde_json 1.0.147 or later: it already uses zmij

## How it is fast

Implements Victor Zverovich's Żmij algorithm for shortest round-trip float-to-decimal conversion, ported line by line from his C++, and writes the digits into a stack Buffer instead of going through core::fmt::Formatter.

## What it costs

- Young crate: first release 2025-12-18

## Problems with the evidence

- First-party benchmark: dtolnay ported zmij and maintains dtoa-benchmark
- Built with -C target-cpu=native for every library
- std's Display never uses exponent notation, so for very large or small values it writes far more characters, which inflates its time
- The zmij README shows only a chart; the numbers are in the dtoa-benchmark table

## Evidence

<details>
<summary>5 quotes from the sources, checked 2026-10-28</summary>

> The following results are measured on a 2025 AMD Ryzen Threadripper 9975WX and 2024 Apple M4 Max, each using Rust 1.97.0:
>
> — [github.com/dtolnay/dtoa-benchmark/blob/a1838f938c130c8cf8d0453dc410…](https://github.com/dtolnay/dtoa-benchmark/blob/a1838f938c130c8cf8d0453dc4109164a0c80a87/README.md)

> <tr><td><a href="https://doc.rust-lang.org/std/fmt/trait.Display.html">libcore</a></td><td align="right">68.8</td><td align="right">1.00&times;</td><td align="right">63.9</td><td align="right">1.00&times;</td></tr>
>
> — [github.com/dtolnay/dtoa-benchmark/blob/a1838f938c130c8cf8d0453dc410…](https://github.com/dtolnay/dtoa-benchmark/blob/a1838f938c130c8cf8d0453dc4109164a0c80a87/README.md)

> <tr><td><a href="https://github.com/dtolnay/ryu">ryu</a></td><td align="right">31.1</td><td align="right">2.21&times;</td><td align="right">26.9</td><td align="right">2.38&times;</td></tr>
>
> — [github.com/dtolnay/dtoa-benchmark/blob/a1838f938c130c8cf8d0453dc410…](https://github.com/dtolnay/dtoa-benchmark/blob/a1838f938c130c8cf8d0453dc4109164a0c80a87/README.md)

> <tr><td><a href="https://github.com/dtolnay/zmij">zmij</a></td><td align="right">8.5</td><td align="right">8.09&times;</td><td align="right">5.9</td><td align="right">10.83&times;</td></tr>
>
> — [github.com/dtolnay/dtoa-benchmark/blob/a1838f938c130c8cf8d0453dc410…](https://github.com/dtolnay/dtoa-benchmark/blob/a1838f938c130c8cf8d0453dc4109164a0c80a87/README.md)

> Switch float-to-string algorithm from Ryū to Żmij for better f32 and f64 serialization performance
>
> — [api.github.com/repos/serde-rs/json/releases/tags/v1.0.147](https://api.github.com/repos/serde-rs/json/releases/tags/v1.0.147)

</details>
