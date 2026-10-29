# zlib-ng via flate2 (libz-ng-sys backend)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Compression](../README.md) › [Deflate, zlib and gzip](README.md)</sub>

**Tier A** (reproducible public benchmark) · 🟡 Holds up in part · Open source · `C (Rust bindings)` · `MIT OR Apache-2.0` · 🟢 active

> flate2's zlib-ng C backend: level-6 deflate 2.1x to 2.7x faster than miniz_oxide; inflate gains are small to none.

| | |
|---|---|
| **Beats** | flate2 1.1 with its default miniz_oxide backend, streaming inflate and deflate in 4096-byte chunks |
| **Claimed result** | Level-6 deflate 37.848 vs 13.772 MB/s on Zen 4 (2.75x) and 47.464 vs 22.074 MB/s on Apple M4 (2.15x); inflate 621.911 vs 533.374 MB/s on Zen 4 (1.17x) and 416.191 vs 416.072 MB/s on M4 (1.00x); author's own run: inflate 578.741 vs 487.149 MB/s (1.19x), level-6 deflate 32.303 vs 11.960 MB/s (2.70x) (ratios derived from page data). |
| **Workload** | silesia-small.tar through flate2's streaming API in 4096-byte chunks: inflate, and deflate at levels 1, 6 and 9; mean wall time per run. |
| **Metric** | throughput |
| **Hardware** | AMD Zen 4 desktop and Apple M4; unnamed x86_64 Linux machine (second run) |
| **OS** | Linux x86_64; macOS arm64 |
| **Toolchain** | rustc 1.97.0-nightly (2026-04-12), LLVM 22.1.2; zlib-ng built from source by libz-ng-sys (run logs list Ubuntu clang 14.0.0 on Zen 4 and Apple clang 21 on M4) |
| **Measured** | 2026-04-13 |
| **Adoption** | 139 stars |
| **Last commit** | 2026-10-28 |
| **Platforms** | Targets zlib-ng supports with a C toolchain; needs CMake unless the experimental cc-based build feature is enabled |
| **Install** | cargo add flate2 --no-default-features --features zlib-ng,runtime_detection |
| **Links** | [repository](https://github.com/rust-lang/libz-sys) · [documentation](https://docs.rs/libz-ng-sys) · [benchmark](https://github.com/rust-lang/flate2-rs/pull/545) |

## Verdict

Compression gains are large (2.1x to 2.7x at level 6), but inflate is only 1.17x faster on Zen 4 and no faster on Apple M4, where it ties miniz_oxide at 416 MB/s. zlib-rs matches or beats it on inflate without a C build.

**Why tier A:** Public flate2 harness run by developers outside the zlib-ng project, with rustc and LLVM versions and CPUs stated.

## Use it when

- Compression throughput matters and a C toolchain with CMake is already in the build
- Other parts of the dependency graph already link zlib-ng

## Skip it when

- You mostly decompress: zlib-rs is faster on inflate in the same runs and needs no C build
- You cross-compile to targets where building C with CMake is painful (wasm, some Windows setups)
- Tests pin exact compressed bytes or sizes

## How it is fast

zlib-ng is a modernized zlib fork with SIMD checksums, SIMD string matching and faster deflate strategies per level, selected by runtime CPU detection. flate2 links it natively (not in zlib-compat mode) through libz-ng-sys.

## What it costs

- C code parsing untrusted input, outside Rust's memory-safety checks
- Build needs CMake or an experimental cc-based feature
- Different compression ratios from miniz_oxide at some levels (1.846 vs 1.991 at level 1 in these runs)

## Problems with the evidence

- One input file (silesia-small.tar)
- Nightly rustc in both runs; the C side's compiler differed between machines
- On Apple M4 inflate was a tie with miniz_oxide

## Evidence

<details>
<summary>16 quotes from the sources, checked 2026-10-28</summary>

> run.sh output with fdeflate on desktop Zen4
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> target/release/flate2_bench_miniz_oxide mean runtime 0.028137s (stdev of 0.000248) at 533.374 MB/s, ratio of 0.404
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> target/release/flate2_bench_zlib_ng mean runtime 0.024131s (stdev of 0.000032) at 621.911 MB/s, ratio of 0.404
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> target/release/flate2_bench_miniz_oxide mean runtime 0.443965s (stdev of 0.000692) at 13.772 MB/s, ratio of 2.454
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> target/release/flate2_bench_zlib_ng mean runtime 0.162743s (stdev of 0.000310) at 37.848 MB/s, ratio of 2.436
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> run.sh output with fdeflate on Apple M4
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> target/release/flate2_bench_miniz_oxide mean runtime 0.036069s (stdev of 0.010199) at 416.072 MB/s, ratio of 0.404
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> target/release/flate2_bench_zlib_ng mean runtime 0.036059s (stdev of 0.012593) at 416.191 MB/s, ratio of 0.404
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> target/release/flate2_bench_miniz_oxide mean runtime 0.276999s (stdev of 0.001565) at 22.074 MB/s, ratio of 2.454
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> target/release/flate2_bench_zlib_ng mean runtime 0.129773s (stdev of 0.002081) at 47.464 MB/s, ratio of 2.436
>
> — [github.com/rust-lang/flate2-rs/pull/545](https://github.com/rust-lang/flate2-rs/pull/545)

> target/release/flate2_bench_miniz_oxide mean runtime 0.030806s (stdev of 0.000302) at 487.149 MB/s, ratio of 0.404
>
> — [github.com/rust-lang/flate2-rs/pull/542](https://github.com/rust-lang/flate2-rs/pull/542)

> target/release/flate2_bench_zlib_ng mean runtime 0.025931s (stdev of 0.000179) at 578.741 MB/s, ratio of 0.404
>
> — [github.com/rust-lang/flate2-rs/pull/542](https://github.com/rust-lang/flate2-rs/pull/542)

> target/release/flate2_bench_miniz_oxide mean runtime 0.511247s (stdev of 0.000757) at 11.960 MB/s, ratio of 2.454
>
> — [github.com/rust-lang/flate2-rs/pull/542](https://github.com/rust-lang/flate2-rs/pull/542)

> target/release/flate2_bench_zlib_ng mean runtime 0.190680s (stdev of 0.001263) at 32.303 MB/s, ratio of 2.436
>
> — [github.com/rust-lang/flate2-rs/pull/542](https://github.com/rust-lang/flate2-rs/pull/542)

> the zlib-ng C library can be slightly faster in certain cases
>
> — [github.com/rust-lang/flate2-rs/blob/9d13b79e4c0c1d397c58fd3deac32eb…](https://github.com/rust-lang/flate2-rs/blob/9d13b79e4c0c1d397c58fd3deac32ebae5bffb5b/README.md)

> Building zlib-ng requires `cmake` unless the `zlib-ng-no-cmake-experimental-community-maintained` feature is enabled
>
> — [github.com/rust-lang/libz-sys/blob/66a0e1d4f51c4e83f36bdcd7075863fb…](https://github.com/rust-lang/libz-sys/blob/66a0e1d4f51c4e83f36bdcd7075863fb45750e9d/README.md)

</details>
