# libdeflater (libdeflate bindings)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Compression](../README.md) › [Deflate, zlib and gzip](README.md)</sub>

**Tier C** (first-party numbers) · 🟡 Holds up in part · Open source · `C (Rust bindings)` · `Apache-2.0` · 🟢 active

> Bindings to libdeflate for whole-buffer gzip/zlib; 2019 README shows 2-3x over flate2's old C miniz default.

| | |
|---|---|
| **Beats** | flate2 with no feature flags in 2019 (the README calls it the miniz implementation; miniz_oxide results sit on a separate branch), whole-buffer compress and decompress |
| **Claimed result** | Compression: average 2-3x on the Calgary and Canterbury corpora (book1: 32697 vs 10377 us, 3.2x; kennedy.xls 7.3x); decompression: average around 2x (book1 1906 vs 768 us, 2.5x; kennedy.xls only 1.4x, 1241 vs 911 us). |
| **Workload** | Each file of the Calgary and Canterbury corpora (3 KB to 1 MB), read into memory, compressed at default level and decompressed in one call; single-threaded. |
| **Metric** | time |
| **Measured** | 2019-10 |
| **Adoption** | 48 stars |
| **Last commit** | 2026-09-17 |
| **Platforms** | Targets with a C compiler; freestanding feature for wasm32-unknown-unknown |
| **Install** | cargo add libdeflater |
| **Links** | [repository](https://github.com/libdeflater/libdeflater) · [documentation](https://docs.rs/libdeflater) · [benchmark](https://github.com/libdeflater/libdeflater/blob/260e1e9a333e32e769855c1ad1f203289e0a3c8d/README.md) |

## Verdict

The 2x to 3x figures are from 2019 against flate2's old C miniz default; flate2 now defaults to miniz_oxide and offers zlib-rs and zlib-ng backends that close much of the gap. The win also only applies to whole-buffer work, since libdeflate has no streaming API.

**Why tier C:** First-party README numbers with a public cargo bench suite, but no CPU, OS or rustc version stated.

## Use it when

- Inputs arrive whole and their decompressed size is known or bounded (BAM blocks, object-store chunks, game packets)
- You can link C code

## Skip it when

- You need streaming compression or decompression; the README itself points general users to flate2
- You want a pure-Rust build

## How it is fast

libdeflate drops zlib's streaming state machine and works on whole buffers, which lets it use larger Huffman decode tables, word-at-a-time copies and SIMD checksums without handling partial input at every step.

## What it costs

- C dependency handling untrusted input
- Caller must size the output buffer for decompression

## Problems with the evidence

- Benchmarks date from October 2019 and were never rerun
- Baseline was flate2's C miniz backend of the time, not today's miniz_oxide, zlib-rs or zlib-ng
- No hardware, OS or compiler version stated

## Evidence

<details>
<summary>9 quotes from the sources, checked 2026-10-28</summary>

> The benchmark suite indicates a 2-3x speedup accross the Calgary and Canterbury corpuses
>
> — [github.com/libdeflater/libdeflater/blob/260e1e9a333e32e769855c1ad1f…](https://github.com/libdeflater/libdeflater/blob/260e1e9a333e32e769855c1ad1f203289e0a3c8d/README.md)

> Avg. speedup (on this corpus) is around 2-3x
>
> — [github.com/libdeflater/libdeflater/blob/260e1e9a333e32e769855c1ad1f…](https://github.com/libdeflater/libdeflater/blob/260e1e9a333e32e769855c1ad1f203289e0a3c8d/README.md)

> book1 768 3.2 32697 10377
>
> — [github.com/libdeflater/libdeflater/blob/260e1e9a333e32e769855c1ad1f…](https://github.com/libdeflater/libdeflater/blob/260e1e9a333e32e769855c1ad1f203289e0a3c8d/README.md)

> kennedy.xls 1029 7.3 46598 6427
>
> — [github.com/libdeflater/libdeflater/blob/260e1e9a333e32e769855c1ad1f…](https://github.com/libdeflater/libdeflater/blob/260e1e9a333e32e769855c1ad1f203289e0a3c8d/README.md)

> Avg. speedup (on this corpus) is around 2x.
>
> — [github.com/libdeflater/libdeflater/blob/260e1e9a333e32e769855c1ad1f…](https://github.com/libdeflater/libdeflater/blob/260e1e9a333e32e769855c1ad1f203289e0a3c8d/README.md)

> book1 768 2.5 1906 768
>
> — [github.com/libdeflater/libdeflater/blob/260e1e9a333e32e769855c1ad1f…](https://github.com/libdeflater/libdeflater/blob/260e1e9a333e32e769855c1ad1f203289e0a3c8d/README.md)

> kennedy.xls 1029 1.4 1241 911
>
> — [github.com/libdeflater/libdeflater/blob/260e1e9a333e32e769855c1ad1f…](https://github.com/libdeflater/libdeflater/blob/260e1e9a333e32e769855c1ad1f203289e0a3c8d/README.md)

> Comparison made against `flate2` with no feature flags (i.e. `miniz` implementation).
>
> — [github.com/libdeflater/libdeflater/blob/260e1e9a333e32e769855c1ad1f…](https://github.com/libdeflater/libdeflater/blob/260e1e9a333e32e769855c1ad1f203289e0a3c8d/README.md)

> You should use something like [flate2](https://github.com/alexcrichton/flate2-rs) if you want a general-purpose deflate library that supports streaming.
>
> — [github.com/libdeflater/libdeflater/blob/260e1e9a333e32e769855c1ad1f…](https://github.com/libdeflater/libdeflater/blob/260e1e9a333e32e769855c1ad1f203289e0a3c8d/README.md)

</details>
