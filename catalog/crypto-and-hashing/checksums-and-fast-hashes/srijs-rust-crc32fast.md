# crc32fast (SIMD CRC-32/IEEE)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Crypto and hashing](../README.md) › [Checksums and fast hashes](README.md)</sub>

**Tier C** (first-party numbers) · 🟡 Holds up in part · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> CRC-32/IEEE with runtime-selected PCLMULQDQ or ARMv8 CRC instructions; 7.3 GB/s in its own table, far ahead of table-driven CRC.

| | |
|---|---|
| **Beats** | crc crate 1.8.1 in the README table; today's crc 3.x defaults to a 256-entry table |
| **Claimed result** | 7314 MB/s (pclmulqdq) and 1499 MB/s (baseline table path) vs 207 MB/s for crc 1.8.1 (35x and 7x, derived from page data). Against crc 3.x measured on another machine: 0.516 GiB/s default bytewise, 3.30 GiB/s Slice16 (about 13x and 2x, derived from page data). |
| **Workload** | CRC-32 (IEEE) over one buffer; size not stated (the ns/iter and MB/s columns imply about 1 KB, derived from page data). |
| **Metric** | throughput |
| **Adoption** | 342 stars |
| **Last commit** | 2026-10-20 |
| **Platforms** | x86/x86_64 (SSE + PCLMULQDQ; VPCLMULQDQ with AVX2/AVX-512 on Rust 1.89+), aarch64 CRC32 instructions, table fallback elsewhere; no_std without SIMD |
| **Install** | cargo add crc32fast |
| **Links** | [repository](https://github.com/srijs/rust-crc32fast) · [documentation](https://docs.rs/crc32fast) · [benchmark](https://github.com/srijs/rust-crc32fast/blob/master/README.md) |

## Verdict

The README's 35x lead (7314 MB/s vs 207 MB/s) is against crc 1.8.1. crc 3.4 reports 0.516 GiB/s for its default bytewise CRC-32 and 3.30 GiB/s with Slice16 on a Ryzen 7 3800X, so against the current crate the gap is roughly 13x for the default and 2x for Slice16, comparing numbers from different machines. The PCLMULQDQ path is still the fastest of the three.

**Why tier C:** First-party table from crc32fast 1.0.0 with no CPU, OS, rustc or input size stated.

## Use it when

- zip, gzip, PNG and other formats that need CRC-32/IEEE
- Checksumming large buffers on x86_64 or aarch64

## Skip it when

- You need CRC-32C (Castagnoli) or another polynomial: crc32fast computes CRC-32/IEEE only
- no_std builds, where runtime detection and so the SIMD paths are unavailable

## How it is fast

On x86 it folds the input with carry-less multiplication (PCLMULQDQ, or VPCLMULQDQ on 256- and 512-bit registers), processing many bytes per instruction instead of a table lookup per byte. On aarch64 it uses the hardware crc32 instructions with 3-way interleaving to hide latency. Hasher::new picks the implementation at runtime; the fallback still processes 16 bytes per iteration.

## What it costs

- unsafe SIMD intrinsics, fuzzed and run under AddressSanitizer in CI per the README

## Problems with the evidence

- The table is from crc32fast 1.0.0; the newer AVX2/AVX-512 VPCLMULQDQ paths have no published numbers
- The baseline crc 1.8.1 is long superseded
- No CPU stated; the crc 3.x numbers come from a different machine

## Evidence

<details>
<summary>6 quotes from the sources, checked 2026-10-28</summary>

> | [crc](https://crates.io/crates/crc) | 1.8.1   | n/a       | 4,926   | 207  |
>
> — [github.com/srijs/rust-crc32fast/blob/master/README.md](https://github.com/srijs/rust-crc32fast/blob/master/README.md)

> | crc32fast (this crate)              | 1.0.0   | baseline  | 683     | 1499 |
>
> — [github.com/srijs/rust-crc32fast/blob/master/README.md](https://github.com/srijs/rust-crc32fast/blob/master/README.md)

> | crc32fast (this crate)              | 1.0.0   | pclmulqdq | 140     | 7314 |
>
> — [github.com/srijs/rust-crc32fast/blob/master/README.md](https://github.com/srijs/rust-crc32fast/blob/master/README.md)

> with AMD Ryzen 7 3800X
>
> — [github.com/mrhooray/crc-rs/blob/master/README.md](https://github.com/mrhooray/crc-rs/blob/master/README.md)

> | 32    | 0.111   | 0.516    | 3.30    |
>
> — [github.com/mrhooray/crc-rs/blob/master/README.md](https://github.com/mrhooray/crc-rs/blob/master/README.md)

> is the default implementation, but this can be overridden
>
> — [github.com/mrhooray/crc-rs/blob/master/README.md](https://github.com/mrhooray/crc-rs/blob/master/README.md)

</details>
