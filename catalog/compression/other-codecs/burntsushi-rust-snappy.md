# snap (pure-Rust Snappy)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Compression](../README.md) › [Zstandard, LZ4, Snappy and Brotli](README.md)</sub>

**Tier C** (first-party numbers) · ✅ Holds up · Open source · `Rust` · `BSD-3-Clause` · 🟢 active

> Pure-Rust Snappy with raw and frame formats; measured within about 10% of C++ Snappy either way on x86_64.

| | |
|---|---|
| **Beats** | Snappy C++ 1.1.8 through the same benchmark harness |
| **Claimed result** | Parity with C++ Snappy: decompress txt2 1001.8 vs 916.6 MB/sec (1.09x faster), txt4 930.2 vs 857.2 MB/sec; compress gaviota 602.3 vs 563.5 MB/sec; but decompress pdf 15.3 vs 16.8 GB/sec (1.10x slower) and compress urls 542.0 vs 566.3 MB/sec. |
| **Workload** | Snappy's own microbenchmark corpus (html, urls, jpg, pdf, text, protobuf files), raw block compress and decompress. |
| **Metric** | throughput |
| **Hardware** | Intel i7-6900K |
| **Measured** | 2020-02-14 |
| **Adoption** | 515 stars |
| **Last commit** | 2026-07-15 |
| **Platforms** | Any Rust target; parity measured on x86_64 |
| **Install** | cargo add snap |
| **Links** | [repository](https://github.com/BurntSushi/rust-snappy) · [documentation](https://docs.rs/snap) · [benchmark](https://github.com/BurntSushi/rust-snappy/blob/26973271c231caca7619107a4b9033a1cbfd3139/README.md) |

## Verdict

The claim is parity, and the table shows it: per-file results land within about 10% of C++ in both directions, with snap ahead on text decompression and behind on pdf decompression. It is a pure-Rust drop-in, not a speedup.

**Why tier C:** First-party table from a public harness with the CPU named, but no rustc version or flags stated.

## Use it when

- You need Snappy without a C++ toolchain
- You need the Snappy frame format, which the C++ binding crate lacks

## Skip it when

- You are looking for a speedup over the C++ library; there is none to find

## How it is fast

A close port of the C++ algorithm with unaligned word loads and copies, plus a hand-written frame format. It matches C++ rather than beating it.

## What it costs

- Snappy itself trades ratio for speed, and LZ4 decoders beat it in lz4_flex's benchmark; pick snap for Snappy compatibility, not speed

## Problems with the evidence

- Run in 2020 against Snappy 1.1.8 on an Intel i7-6900K (2016); no newer comparison published
- No rustc version or flags stated

## Evidence

<details>
<summary>8 quotes from the sources, checked 2026-10-28</summary>

> The performance of this implementation should roughly match the performance of the C++ implementation on x86_64.
>
> — [github.com/BurntSushi/rust-snappy/blob/26973271c231caca7619107a4b90…](https://github.com/BurntSushi/rust-snappy/blob/26973271c231caca7619107a4b9033a1cbfd3139/README.md)

> decompress/uflat07_txt2 1.09 130.2±0.93µs 916.6 MB/sec 1.00 119.2±0.96µs 1001.8 MB/sec
>
> — [github.com/BurntSushi/rust-snappy/blob/26973271c231caca7619107a4b90…](https://github.com/BurntSushi/rust-snappy/blob/26973271c231caca7619107a4b9033a1cbfd3139/README.md)

> decompress/uflat09_txt4 1.09 536.1±3.47µs 857.2 MB/sec 1.00 494.0±5.05µs 930.2 MB/sec
>
> — [github.com/BurntSushi/rust-snappy/blob/26973271c231caca7619107a4b90…](https://github.com/BurntSushi/rust-snappy/blob/26973271c231caca7619107a4b9033a1cbfd3139/README.md)

> compress/zflat11_gaviota 1.07 311.9±4.27µs 563.5 MB/sec 1.00 291.9±1.86µs 602.3 MB/sec
>
> — [github.com/BurntSushi/rust-snappy/blob/26973271c231caca7619107a4b90…](https://github.com/BurntSushi/rust-snappy/blob/26973271c231caca7619107a4b9033a1cbfd3139/README.md)

> decompress/uflat04_pdf 1.00 5.7±0.05µs 16.8 GB/sec 1.10 6.2±0.07µs 15.3 GB/sec
>
> — [github.com/BurntSushi/rust-snappy/blob/26973271c231caca7619107a4b90…](https://github.com/BurntSushi/rust-snappy/blob/26973271c231caca7619107a4b9033a1cbfd3139/README.md)

> compress/zflat01_urls 1.00 1182.3±8.89µs 566.3 MB/sec 1.04 1235.3±11.99µs 542.0 MB/sec
>
> — [github.com/BurntSushi/rust-snappy/blob/26973271c231caca7619107a4b90…](https://github.com/BurntSushi/rust-snappy/blob/26973271c231caca7619107a4b9033a1cbfd3139/README.md)

> These benchmarks were run with Snappy/C++ 1.1.8.
>
> — [github.com/BurntSushi/rust-snappy/blob/26973271c231caca7619107a4b90…](https://github.com/BurntSushi/rust-snappy/blob/26973271c231caca7619107a4b9033a1cbfd3139/README.md)

> Benchmarks were run on an Intel i7-6900K.
>
> — [github.com/BurntSushi/rust-snappy/blob/26973271c231caca7619107a4b90…](https://github.com/BurntSushi/rust-snappy/blob/26973271c231caca7619107a4b9033a1cbfd3139/README.md)

</details>
