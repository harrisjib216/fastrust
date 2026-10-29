# fdeflate (safe-Rust inflate, PNG-tuned)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Compression](../README.md) › [Deflate, zlib and gzip](README.md)</sub>

**Tier A** (reproducible public benchmark) · 🟡 Holds up in part · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Safe-Rust deflate decoder from image-rs; inflated 1.5x to 1.6x faster than miniz_oxide in a flate2 prototype.

| | |
|---|---|
| **Beats** | flate2 1.1 with its default miniz_oxide backend, streaming inflate in 4096-byte chunks |
| **Claimed result** | Inflate 815.348 vs 533.374 MB/s on Zen 4 (1.53x) and 677.090 vs 416.072 MB/s on Apple M4 (1.63x), also ahead of zlib-rs (721.516 and 527.478 MB/s); level-6 deflate 23.136 vs 13.772 MB/s on Zen 4 (1.68x) (ratios derived from page data). |
| **Workload** | silesia-small.tar through flate2's streaming API in 4096-byte chunks, using a proof-of-concept fdeflate backend for flate2. |
| **Metric** | throughput |
| **Hardware** | AMD Zen 4 desktop; Apple M4 |
| **OS** | Linux x86_64; macOS arm64 |
| **Toolchain** | rustc 1.97.0-nightly (2026-04-12), LLVM 22.1.2, release build |
| **Measured** | 2026-04-13 |
| **Adoption** | 38 stars |
| **Last commit** | 2026-05-31 |
| **Platforms** | Any Rust target; checksums come from simd-adler32 |
| **Install** | cargo add fdeflate |
| **Links** | [repository](https://github.com/image-rs/fdeflate) · [documentation](https://docs.rs/fdeflate) · [benchmark](https://github.com/rust-lang/flate2-rs/pull/545) |

## Verdict

The decoder ran 1.5x to 1.6x faster than miniz_oxide and ahead of zlib-rs on Zen 4 and Apple M4, but only through a proof-of-concept flate2 adapter on a patched fork. The published 0.3.x crate has its own API and only an ultra-fast compression mode, so the general deflate numbers are not yet available to users.

**Why tier A:** Public harness branch (Shnatsel/flate2_bench, fdeflate branch) with rustc version, LLVM version and CPUs stated.

## Use it when

- You decode PNG or other zlib streams in memory and can call fdeflate's own API
- You want a faster inflate without adding unsafe code or C to the dependency tree

## Skip it when

- You need a flate2 backend today: the adapter in flate2 PR 545 is a closed proof of concept
- You need general compression levels 1 to 9: the released 0.3.x encoder only has an ultra-fast PNG mode
- You rely on preset dictionaries, which fdeflate lacks

## How it is fast

Larger Huffman lookup tables (up to 4096 entries) sized for modern L1 caches, decoding two literals per table lookup, and cheaper table construction with fewer random writes. It does this without unsafe code in the crate itself.

## What it costs

- Compression ratio at level 1 was 1.555 against 1.991 for miniz_oxide in the same run
- No streaming flate2 integration yet; the prototype needed an extra copy for the lookback window

## Problems with the evidence

- Numbers come from an unmerged prototype that depends on a fork exposing private fdeflate state
- Measured by a member of the image-rs organization, which maintains fdeflate
- One input file; compression heuristics are tuned for PNG and gave slightly worse ratios on silesia

## Evidence

<details>
<summary>14 quotes from the sources, checked 2026-10-28</summary>

> run.sh output with fdeflate on desktop Zen4
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> rustc 1.97.0-nightly (14196dbfa 2026-04-12)
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> target/release/flate2_bench_miniz_oxide mean runtime 0.028137s (stdev of 0.000248) at 533.374 MB/s, ratio of 0.404
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> target/release/flate2_bench_fdeflate mean runtime 0.018406s (stdev of 0.000335) at 815.348 MB/s, ratio of 0.404
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> target/release/flate2_bench_zlib_rs mean runtime 0.020800s (stdev of 0.000088) at 721.516 MB/s, ratio of 0.404
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> target/release/flate2_bench_miniz_oxide mean runtime 0.443965s (stdev of 0.000692) at 13.772 MB/s, ratio of 2.454
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> target/release/flate2_bench_fdeflate mean runtime 0.267347s (stdev of 0.000694) at 23.136 MB/s, ratio of 2.426
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> target/release/flate2_bench_fdeflate mean runtime 0.040613s (stdev of 0.001354) at 237.573 MB/s, ratio of 1.555
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> run.sh output with fdeflate on Apple M4
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> target/release/flate2_bench_miniz_oxide mean runtime 0.036069s (stdev of 0.010199) at 416.072 MB/s, ratio of 0.404
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> target/release/flate2_bench_fdeflate mean runtime 0.022164s (stdev of 0.006995) at 677.090 MB/s, ratio of 0.404
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> target/release/flate2_bench_zlib_rs mean runtime 0.028451s (stdev of 0.009108) at 527.478 MB/s, ratio of 0.404
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> This is a very early proof-of-concept,
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> In the 0.3.x series, the ultra-fast mode is the only supported compression mode.
>
> — [github.com/image-rs/fdeflate/blob/0554737242b840de9d48bdcc585c33da2…](https://github.com/image-rs/fdeflate/blob/0554737242b840de9d48bdcc585c33da2ac80ab7/README.md)

</details>
