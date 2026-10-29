# ryu (Ryū float-to-string)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Data formats](../README.md) › [Numbers to and from text](README.md)</sub>

**Tier A** (reproducible public benchmark) · ✅ Holds up · Open source · `Rust` · `Apache-2.0 OR BSL-1.0` · 🟢 active

> Ryū shortest float-to-string: 2.2x-2.4x faster than std's float Display, now overtaken by zmij.

| | |
|---|---|
| **Beats** | std's Display for f32/f64 |
| **Claimed result** | README: approximately 2-5x faster than std. dtoa-benchmark: 31.1 vs 68.8 ns per f64 (2.21x) on Threadripper 9975WX, 26.9 vs 63.9 ns (2.38x) on M4 Max; zmij 8.5 ns and 5.9 ns on the same machines. |
| **Workload** | Random f64 bit patterns truncated to 1-17 significant digits; fastest pass per precision group, averaged across groups. |
| **Metric** | latency |
| **Hardware** | AMD Ryzen Threadripper 9975WX; Apple M4 Max |
| **Toolchain** | Rust 1.97.0, --release, -C target-cpu=native (repo .cargo/config.toml) |
| **Measured** | 2026-07-12 |
| **Adoption** | 709 stars |
| **Last commit** | 2026-08-06 |
| **Platforms** | portable; no_std |
| **Install** | cargo add ryu |
| **Links** | [repository](https://github.com/dtolnay/ryu) · [documentation](https://docs.rs/ryu) · [benchmark](https://github.com/dtolnay/dtoa-benchmark/blob/a1838f938c130c8cf8d0453dc4109164a0c80a87/README.md) |

## Verdict

The README's approximately 2-5x claim holds at its low end: dtoa-benchmark on Rust 1.97.0 measures 31.1 vs 68.8 ns per f64 (2.21x) on a Threadripper 9975WX and 26.9 vs 63.9 ns (2.38x) on an M4 Max. It is no longer the fastest option: zmij is 3.7x faster in the same table and replaced ryu inside serde_json.

**Why tier A:** Public benchmark with CPUs, Rust 1.97.0 and -C target-cpu=native stated; maintained by ryu's author.

## Use it when

- Existing code on ryu that cannot take a new dependency
- You want the Ryū algorithm's published correctness proof

## Skip it when

- New code: zmij is 3.7x faster in the same benchmark and serde_json moved to it in 1.0.147
- You need std's exact output; ryu writes 1.23e40 where std writes the full digits

## How it is fast

Ryū (Ulf Adams, PLDI 2018) finds the shortest decimal that round-trips using precomputed power-of-5 tables and 128-bit multiplies, with no big-integer fallback, and writes into a stack buffer instead of through Formatter.

## What it costs

- Output format differs from std for large and small magnitudes

## Problems with the evidence

- The README's own Rust-vs-std numbers are chart-only; the text numbers come from dtoa-benchmark
- std's Display never uses exponent notation, so it writes longer strings for extreme values
- All libraries built with -C target-cpu=native

## Evidence

<details>
<summary>7 quotes from the sources, checked 2026-10-28</summary>

> The benchmark shows Ryū approximately 2-5x faster than the standard library across a range of f32 and f64 inputs.
>
> — [github.com/dtolnay/ryu](https://github.com/dtolnay/ryu)

> *ryu:* 1.23e40, *std:* 12300000000000000000000000000000000000000
>
> — [github.com/dtolnay/ryu](https://github.com/dtolnay/ryu)

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
