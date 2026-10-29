# const-hex

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Data formats](../README.md) › [Base64, hex and encoding](README.md)</sub>

**Tier A** (reproducible public benchmark) · ✅ Holds up · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Drop-in replacement for the hex crate with SIMD and const-eval, measured 6x-99x faster than hex and up to 158x faster than std formatting.

| | |
|---|---|
| **Beats** | hex 0.4.3 encode/decode, and std formatting with write!(f, "{byte:02x}") per byte |
| **Claimed result** | encode 7.06 vs 102.11 ns (32 B) to 30,716.17 vs 3,048,496.30 ns (1 MiB), 14x-99x; decode 17.57 vs 107.13 ns to 108,937.41 vs 6,409,293.90 ns, 6.1x-59x; encode_to_slice 1.57 vs 12.11 ns to 32,631.90 vs 518,929.65 ns, 7.7x-16x; format vs std 10.01 vs 373.63 ns (32 B), 26.68 vs 2,885.37 ns (256 B), 78,056.40 vs 12,348,138.40 ns (1 MiB) (ratios derived from page data) |
| **Workload** | libtest #[bench] on 32 B, 256 B, 2 KiB, 16 KiB, 128 KiB and 1 MiB inputs: allocating encode/decode, encode/decode into slices, and Display formatting into a Vec. |
| **Metric** | time |
| **Hardware** | AMD Ryzen 9 7950X |
| **OS** | Linux (x86_64-unknown-linux-gnu) |
| **Toolchain** | rustc 1.83.0-nightly (9ff5fc4ff 2024-10-03), cargo bench |
| **Measured** | 2024-10-06 |
| **Adoption** | 66 stars |
| **Last commit** | 2026-10-28 |
| **Platforms** | x86/x86_64 (SIMD routines adapted from faster-hex) and other platforms; const fn encoding at compile time |
| **Install** | cargo add const-hex |
| **Links** | [repository](https://github.com/DaniPopes/const-hex) · [documentation](https://docs.rs/const-hex) · [benchmark](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md) |

## Verdict

Its own run backs the claim: encode 14x-99x and decode 6x-59x faster than hex across 32 B-1 MiB, and formatting 37x faster than per-byte std formatting at 32 bytes, rising above 100x from 256 bytes. The to_slice variants, where hex avoids allocation, show smaller 7.7x-16x encode gaps.

**Why tier A:** Public cargo bench code with the CPU and rustc nightly stated; first-party.

## Use it when

- Replacing hex 0.4 without changing call sites
- Formatting byte arrays as hex in Display/Debug impls (hashes, addresses) instead of write!(f, "{:02x}") per byte
- Hex constants decoded at compile time

## Skip it when

- Tiny inputs written into a preallocated slice, where the gap over hex is closer to 7x-8x and may not matter

## How it is fast

Adapts faster-hex's x86 check and decode routines and has its own code paths for other platforms. A const-generic Buffer, modeled on itoa's, formats into a fixed-size stack buffer, avoiding the per-byte Formatter calls that make std formatting slow.

## What it costs

- SIMD paths use unsafe
- MSRV 1.64 for the released 1.19.3; the main branch raised it to 1.85

## Problems with the evidence

- First-party numbers on one machine with a nightly toolchain
- The std comparison is per-byte write!("{:02x}"), the slowest common idiom, not a hand-written std loop

## Evidence

<details>
<summary>20 quotes from the sources, checked 2026-10-28</summary>

> This crate is 10 to 50 times faster than [`hex`] in encoding and decoding, and 100+ times faster than `libstd` in formatting.
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

> The following benchmarks were ran on an AMD Ryzen 9 7950X, compiled with `1.83.0-nightly (9ff5fc4ff 2024-10-03)` on `x86_64-unknown-linux-gnu`.
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

> test encode::const_hex::bench1_32b ... bench: 7.06 ns/iter (+/- 0.15)
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

> test encode::const_hex::bench6_1m ... bench: 30,716.17 ns/iter (+/- 795.98)
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

> test encode::hex::bench1_32b ... bench: 102.11 ns/iter (+/- 1.75)
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

> test encode::hex::bench6_1m ... bench: 3,048,496.30 ns/iter (+/- 223,992.59)
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

> test decode::const_hex::bench1_32b ... bench: 17.57 ns/iter (+/- 0.53)
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

> test decode::const_hex::bench6_1m ... bench: 108,937.41 ns/iter (+/- 6,453.24)
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

> test decode::hex::bench1_32b ... bench: 107.13 ns/iter (+/- 7.28)
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

> test decode::hex::bench6_1m ... bench: 6,409,293.90 ns/iter (+/- 102,747.28)
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

> test encode_to_slice::const_hex::bench1_32b ... bench: 1.57 ns/iter (+/- 0.01)
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

> test encode_to_slice::const_hex::bench6_1m ... bench: 32,631.90 ns/iter (+/- 1,135.39)
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

> test encode_to_slice::hex::bench1_32b ... bench: 12.11 ns/iter (+/- 0.31)
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

> test encode_to_slice::hex::bench6_1m ... bench: 518,929.65 ns/iter (+/- 2,202.04)
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

> test format::const_hex::bench1_32b ... bench: 10.01 ns/iter (+/- 0.09)
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

> test format::const_hex::bench2_256b ... bench: 26.68 ns/iter (+/- 0.49)
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

> test format::const_hex::bench6_1m ... bench: 78,056.40 ns/iter (+/- 1,584.60)
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

> test format::std::bench1_32b ... bench: 373.63 ns/iter (+/- 3.41)
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

> test format::std::bench2_256b ... bench: 2,885.37 ns/iter (+/- 43.81)
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

> test format::std::bench6_1m ... bench: 12,348,138.40 ns/iter (+/- 291,827.38)
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

</details>
