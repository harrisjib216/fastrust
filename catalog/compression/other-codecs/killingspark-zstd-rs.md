# ruzstd (pure-Rust zstd)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Compression](../README.md) › [Zstandard, LZ4, Snappy and Brotli](README.md)</sub>

**Tier C** (first-party numbers) · ❌ Doesn't hold up · Open source · `Rust` · `MIT` · 🟢 active

> Pure-Rust zstd decoder and partial encoder; 1.4x to 3.5x slower than the C library behind the zstd crate.

| | |
|---|---|
| **Beats** | The zstd crate's bindings to the C reference zstd library |
| **Claimed result** | None: decoder about 3.5 times slower than C zstd on enwik9 and about 1.4 times slower on less compressible data; libzstd-rs-sys decompression a few percent (~3%) slower than C by default. |
| **Workload** | Decoding enwik9.zst and an Ubuntu ISO compressed with zstd, from a ramfs. |
| **Metric** | time |
| **Measured** | 2019 |
| **Adoption** | 455 stars |
| **Last commit** | 2026-10-16 |
| **Platforms** | Any Rust target (pure Rust, no C toolchain) |
| **Install** | cargo add ruzstd |
| **Links** | [repository](https://github.com/KillingSpark/zstd-rs) · [documentation](https://docs.rs/ruzstd) · [benchmark](https://github.com/KillingSpark/zstd-rs/blob/da53f315692c1340c5e4d0998e249c1ff2372053/Readme.md) |

## Verdict

No pure-Rust zstd beats the C bindings. ruzstd's own README puts its decoder 1.4x to 3.5x slower, its encoder lacks the default level, and Trifecta's newer libzstd-rs-sys port is about 3% slower than C by default.

**Why tier C:** First-party measurements taken with the time utility, with no hardware or compiler stated.

## Use it when

- You need zstd decoding on wasm, no_std or targets without a C toolchain and can accept slower decoding

## Skip it when

- Speed matters: use the zstd crate (C bindings)
- You need compression levels beyond Fastest; Default and higher are not implemented

## How it is fast

It is not faster. It is a clean-room Rust implementation of RFC 8878 without the C library's hand-tuned decoding loops; its value is portability and no C toolchain.

## What it costs

- Encoder supports only uncompressed and Fastest (about level 1) blocks
- libzstd-rs-sys can match C only with an unsafe-performance-experimental flag that drops four bounds checks

## Problems with the evidence

- The README's speed figures date from 2019 and were not rerun for later releases
- Single-run time measurements with no hardware named

## Evidence

<details>
<summary>7 quotes from the sources, checked 2026-10-28</summary>

> In terms of speed, `ruzstd` is behind the original C implementation
>
> — [github.com/KillingSpark/zstd-rs/blob/da53f315692c1340c5e4d0998e249c…](https://github.com/KillingSpark/zstd-rs/blob/da53f315692c1340c5e4d0998e249c1ff2372053/Readme.md)

> decoding the same enwik9.zst file from a ramfs
>
> — [github.com/KillingSpark/zstd-rs/blob/da53f315692c1340c5e4d0998e249c…](https://github.com/KillingSpark/zstd-rs/blob/da53f315692c1340c5e4d0998e249c1ff2372053/Readme.md)

> my decoder is about 3.5 times slower.
>
> — [github.com/KillingSpark/zstd-rs/blob/da53f315692c1340c5e4d0998e249c…](https://github.com/KillingSpark/zstd-rs/blob/da53f315692c1340c5e4d0998e249c1ff2372053/Readme.md)

> my decoder comes close to only being 1.4 times slower.
>
> — [github.com/KillingSpark/zstd-rs/blob/da53f315692c1340c5e4d0998e249c…](https://github.com/KillingSpark/zstd-rs/blob/da53f315692c1340c5e4d0998e249c1ff2372053/Readme.md)

> It also provides a compressor which is usable, but it does not yet reach the speed, ratio or configurability of the original zstd library.
>
> — [github.com/KillingSpark/zstd-rs/blob/da53f315692c1340c5e4d0998e249c…](https://github.com/KillingSpark/zstd-rs/blob/da53f315692c1340c5e4d0998e249c1ff2372053/Readme.md)

> By default decompression performance of our implementation is a few percent slower than the C reference implementation.
>
> — [trifectatech.org/blog/announcing-zstandard-in-rust/](https://trifectatech.org/blog/announcing-zstandard-in-rust/)

> For most users a ~3% performance regression is likely an acceptable price to pay for increased memory safety.
>
> — [trifectatech.org/blog/announcing-zstandard-in-rust/](https://trifectatech.org/blog/announcing-zstandard-in-rust/)

</details>
