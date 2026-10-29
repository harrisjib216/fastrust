# zlib-rs (flate2 zlib-rs backend)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Compression](../README.md) › [Deflate, zlib and gzip](README.md)</sub>

**Tier A** (reproducible public benchmark) · ✅ Holds up · Open source · `Rust` · `Zlib` · 🟢 active

> Rust port of zlib-ng as a flate2 backend; inflates about 1.3x and deflates 1.9x to 2.6x faster than flate2's default miniz_oxide.

| | |
|---|---|
| **Beats** | flate2 1.1 with its default miniz_oxide backend, streaming inflate and deflate in 4096-byte chunks |
| **Claimed result** | Inflate 721.516 vs 533.374 MB/s on Zen 4 (1.35x) and 527.478 vs 416.072 MB/s on Apple M4 (1.27x); level-6 deflate 36.051 vs 13.772 MB/s on Zen 4 (2.62x) and 42.306 vs 22.074 MB/s on M4 (1.92x) (ratios derived from page data). Author's own run: ~30% faster inflate (653.518 vs 487.149 MB/s) and ~3X faster level-6 deflate. |
| **Workload** | silesia-small.tar (about 15 MB) through flate2's streaming API in 4096-byte chunks: inflate of the compressed file, deflate at levels 1, 6 and 9; mean wall time per run. |
| **Metric** | throughput |
| **Hardware** | AMD Zen 4 desktop and Apple M4 (rerun); unnamed x86_64 Linux machine (author's run) |
| **OS** | Linux x86_64 (Zen 4); macOS, arm64-apple-darwin25.4.0 (M4) |
| **Toolchain** | rustc 1.97.0-nightly (2026-04-12), LLVM 22.1.2, release build (rerun); rustc 1.96.0-nightly (2026-04-10) (author's run) |
| **Measured** | 2026-04-13 |
| **Adoption** | 709 stars |
| **Last commit** | 2026-09-17 |
| **Platforms** | x86_64, aarch64, wasm32 and others; SIMD paths chosen at runtime when flate2's runtime_detection feature is on |
| **Install** | cargo add flate2 --no-default-features --features zlib-rs,runtime_detection |
| **Links** | [repository](https://github.com/trifectatechfoundation/zlib-rs) · [documentation](https://docs.rs/zlib-rs) · [benchmark](https://github.com/rust-lang/flate2-rs/pull/545) |

## Verdict

A rerun of the zlib-rs author's harness by a developer outside the project, on Zen 4 and Apple M4, reproduces the gap: about 1.3x faster inflate and 1.9x to 2.6x faster level-6 deflate than miniz_oxide. The author's '~3X' deflate figure rounds up the 2.58x in his own table.

**Why tier A:** Public harness (folkertdev/flate2_bench) rerun by a non-author with rustc version, LLVM version and CPUs stated.

## Use it when

- gzip or zlib work is a measurable share of CPU time in a Rust service or tool
- You want zlib-ng-class speed without a C compiler or CMake in the build
- You compress at the default level 6, where the gap over miniz_oxide is largest

## Skip it when

- Your policy forbids unsafe code in dependencies: zlib-rs uses unsafe for SIMD, allocation and its zlib-compatible state, miniz_oxide uses none
- Tests pin exact compressed bytes or sizes: zlib-rs output differs from miniz_oxide
- Decompression is rare and small, so a 1.3x inflate gain will not show

## How it is fast

zlib-rs follows zlib-ng: SIMD versions of the hot loops (checksums, compare256, slide_hash, match copying) picked once by runtime CPU detection, and zlib-ng's faster match-finding strategy for each level. All allocations happen up front. flate2's default, miniz_oxide, is a port of the smaller miniz library written without unsafe code.

## What it costs

- Unsafe code in a parser of untrusted input; a flate2 reviewer opposed the default switch on these grounds
- Different deflate strategy: at level 1 its ratio was 1.846 against 1.991 for miniz_oxide in the same runs, and zlib-rs issue #500 (open) reports larger level-6 output since 0.5.3
- Raises flate2's MSRV from 1.67 to 1.75 when selected
- Needs default-features = false on flate2 to drop miniz_oxide from the build

## Problems with the evidence

- One input file (silesia-small.tar); a flate2 reviewer asked for more corpora and a geometric mean before any default switch
- Both runs used nightly rustc with LLVM 22
- The author's run does not name its CPU
- The author's '~3X' level-6 deflate claim is 2.58x in his own table (30.881 vs 11.960 MB/s)
- flate2 still defaults to miniz_oxide as of this check; PR 542 to make zlib-rs the default is open

## Evidence

<details>
<summary>21 quotes from the sources, checked 2026-10-28</summary>

> run.sh output with fdeflate on desktop Zen4
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> rustc 1.97.0-nightly (14196dbfa 2026-04-12)
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> target/release/flate2_bench_miniz_oxide mean runtime 0.028137s (stdev of 0.000248) at 533.374 MB/s, ratio of 0.404
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> target/release/flate2_bench_zlib_rs mean runtime 0.020800s (stdev of 0.000088) at 721.516 MB/s, ratio of 0.404
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> target/release/flate2_bench_miniz_oxide mean runtime 0.443965s (stdev of 0.000692) at 13.772 MB/s, ratio of 2.454
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> target/release/flate2_bench_zlib_rs mean runtime 0.170855s (stdev of 0.001357) at 36.051 MB/s, ratio of 2.436
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> run.sh output with fdeflate on Apple M4
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> target/release/flate2_bench_miniz_oxide mean runtime 0.036069s (stdev of 0.010199) at 416.072 MB/s, ratio of 0.404
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> target/release/flate2_bench_zlib_rs mean runtime 0.028451s (stdev of 0.009108) at 527.478 MB/s, ratio of 0.404
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> target/release/flate2_bench_miniz_oxide mean runtime 0.276999s (stdev of 0.001565) at 22.074 MB/s, ratio of 2.454
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> target/release/flate2_bench_zlib_rs mean runtime 0.145596s (stdev of 0.001938) at 42.306 MB/s, ratio of 2.436
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> seeing a ~30% speedup for inflate and ~3X speedup for deflate at level 6 versus miniz_oxide on my machine
>
> — [github.com/rust-lang/flate2-rs/pull/542](https://github.com/rust-lang/flate2-rs/pull/542)

> target/release/flate2_bench_miniz_oxide mean runtime 0.030806s (stdev of 0.000302) at 487.149 MB/s, ratio of 0.404
>
> — [github.com/rust-lang/flate2-rs/pull/542](https://github.com/rust-lang/flate2-rs/pull/542)

> target/release/flate2_bench_zlib_rs mean runtime 0.022964s (stdev of 0.000104) at 653.518 MB/s, ratio of 0.404
>
> — [github.com/rust-lang/flate2-rs/pull/542](https://github.com/rust-lang/flate2-rs/pull/542)

> target/release/flate2_bench_miniz_oxide mean runtime 0.511247s (stdev of 0.000757) at 11.960 MB/s, ratio of 2.454
>
> — [github.com/rust-lang/flate2-rs/pull/542](https://github.com/rust-lang/flate2-rs/pull/542)

> target/release/flate2_bench_zlib_rs mean runtime 0.199463s (stdev of 0.000486) at 30.881 MB/s, ratio of 2.436
>
> — [github.com/rust-lang/flate2-rs/pull/542](https://github.com/rust-lang/flate2-rs/pull/542)

> target/release/flate2_bench_miniz_oxide mean runtime 0.070601s (stdev of 0.000927) at 106.783 MB/s, ratio of 1.991
>
> — [github.com/rust-lang/flate2-rs/pull/542](https://github.com/rust-lang/flate2-rs/pull/542)

> target/release/flate2_bench_zlib_rs mean runtime 0.057523s (stdev of 0.000293) at 141.342 MB/s, ratio of 1.846
>
> — [github.com/rust-lang/flate2-rs/pull/542](https://github.com/rust-lang/flate2-rs/pull/542)

> will bump the MSRV from 1.67 (January 2023) to 1.75 (December 2023)
>
> — [github.com/rust-lang/flate2-rs/pull/542](https://github.com/rust-lang/flate2-rs/pull/542)

> This crate by default uses the `miniz_oxide` crate
>
> — [github.com/rust-lang/flate2-rs/blob/9d13b79e4c0c1d397c58fd3deac32eb…](https://github.com/rust-lang/flate2-rs/blob/9d13b79e4c0c1d397c58fd3deac32ebae5bffb5b/README.md)

> we're now twice as fast as miniz-oxide, and significantly faster than zlib-ng
>
> — [trifectatech.org/blog/fastest-wasm-zlib/](https://trifectatech.org/blog/fastest-wasm-zlib/)

</details>
