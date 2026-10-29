# blake3 (BLAKE3 hash)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Crypto and hashing](../README.md) › [Cryptographic hashes and ciphers](README.md)</sub>

**Tier A** (reproducible public benchmark) · 🟡 Holds up in part · Open source · `Rust + assembly` · `CC0-1.0 OR Apache-2.0 OR Apache-2.0 WITH LLVM-exception` · 🟢 active

> BLAKE3 hash: 14x SHA-256 on a 2019 AVX-512 server without SHA-NI, but slower than hardware SHA-256 on short inputs.

| | |
|---|---|
| **Beats** | SHA-256 (OpenSSL in the published chart); the Rust default is the sha2 crate, which uses SHA-NI or ARMv8 SHA2 instructions when the CPU has them |
| **Claimed result** | 16 KiB input, 1 thread: 6866 MiB/s vs 484 MiB/s for SHA-256 (14x, derived from page data). Third party on a Ryzen 9 3900X with SHA extensions: BLAKE3 almost 20% slower than SHA-256 at 1000 bytes, twice as fast at 100,000 bytes. |
| **Workload** | Single-threaded hashing of 16 KiB inputs (cargo bench '16384$'); third-party run: inputs of 4 bytes to 100,000 bytes. |
| **Metric** | throughput |
| **Hardware** | AWS c5.metal, Intel Cascade Lake-SP 8275CL (AVX-512, no SHA extensions) |
| **OS** | Arch Linux |
| **Toolchain** | BLAKE2 rows built with RUSTFLAGS="-C target-cpu=native"; rustc version not stated |
| **Measured** | 2020 |
| **Adoption** | 6.5k stars |
| **Last commit** | 2026-10-01 |
| **Platforms** | x86/x86_64 (SSE2 to AVX-512, runtime detection), aarch64 NEON, WASM; portable fallback; optional multithreading via rayon |
| **Install** | cargo add blake3 |
| **Links** | [repository](https://github.com/BLAKE3-team/BLAKE3) · [documentation](https://docs.rs/blake3) · [benchmark](https://github.com/BLAKE3-team/BLAKE3-specs/blob/master/benchmarks/bar_chart.py) |

## Verdict

The chart's 14x lead (6866 vs 484 MiB/s at 16 KiB) was measured with AVX-512 BLAKE3 against SHA-256 on a Cascade Lake CPU without SHA extensions. On a CPU with SHA extensions, a third party measured BLAKE3 almost 20% slower than SHA-256 at 1000 bytes and only twice as fast at 100,000 bytes. BLAKE3 still wins clearly on large inputs, on CPUs without SHA-NI, and with multiple threads.

**Why tier A:** Benchmark code, the hardcoded results, the machine (AWS c5.metal, Cascade Lake-SP 8275CL) and the flags are published in BLAKE3-specs.

## Use it when

- Hashing large files or buffers, especially with the rayon feature
- CPUs without SHA extensions (Intel client chips before Ice Lake, the Cascade Lake used in the chart)
- You want a keyed hash, KDF, XOF and verified streaming from one algorithm

## Skip it when

- Interoperability requires SHA-256 (TLS, certificates, Git, most protocols)
- Messages around 1 KB or shorter on CPUs with SHA-NI or ARMv8 SHA2, where SHA-256 can be faster
- Password hashing: BLAKE3 is fast by design

## How it is fast

BLAKE3 is a Merkle tree of 1 KiB chunks over a reduced-round BLAKE2s compression function, so independent chunks hash in parallel across SIMD lanes (up to 16 with AVX-512) and, with the rayon feature, across threads. Short inputs fill only one chunk and cannot use that parallelism, which is where hardware SHA-256 catches up.

## What it costs

- Different digest from SHA-256: not a drop-in where hashes are stored or exchanged
- The rayon feature adds a thread-pool dependency

## Problems with the evidence

- The chart's SHA-256 is OpenSSL, not the sha2 crate
- Cascade Lake has AVX-512, which suits BLAKE3, and lacks SHA extensions, which hurts SHA-256, as the BLAKE3 author notes
- The SHA-NI comparison used .NET's SHA256 on a Ryzen 9 3900X, not the sha2 crate
- Chart data from 2020

## Evidence

<details>
<summary>11 quotes from the sources, checked 2026-10-28</summary>

> ("BLAKE3", 6866),
>
> — [github.com/BLAKE3-team/BLAKE3-specs/blob/master/benchmarks/bar_char…](https://github.com/BLAKE3-team/BLAKE3-specs/blob/master/benchmarks/bar_chart.py)

> ("SHA-256", 484),
>
> — [github.com/BLAKE3-team/BLAKE3-specs/blob/master/benchmarks/bar_char…](https://github.com/BLAKE3-team/BLAKE3-specs/blob/master/benchmarks/bar_chart.py)

> plot.set_title("Performance on AWS c5.metal, 16 KiB input, 1 thread")
>
> — [github.com/BLAKE3-team/BLAKE3-specs/blob/master/benchmarks/bar_char…](https://github.com/BLAKE3-team/BLAKE3-specs/blob/master/benchmarks/bar_chart.py)

> plot.set(xlabel="Speed (MiB/s)")
>
> — [github.com/BLAKE3-team/BLAKE3-specs/blob/master/benchmarks/bar_char…](https://github.com/BLAKE3-team/BLAKE3-specs/blob/master/benchmarks/bar_chart.py)

> with an Intel Cascade Lake-SP 8275CL processor, running Arch Linux.
>
> — [github.com/BLAKE3-team/BLAKE3-specs/blob/master/benchmarks/bar_char…](https://github.com/BLAKE3-team/BLAKE3-specs/blob/master/benchmarks/bar_chart.py)

> SHA-1, SHA-2, and SHA-3 are all provided by OpenSSL.
>
> — [github.com/BLAKE3-team/BLAKE3-specs/blob/master/benchmarks/bar_char…](https://github.com/BLAKE3-team/BLAKE3-specs/blob/master/benchmarks/bar_chart.py)

> Around 1000 bytes, strangely BLAKE3 is performing almost 20% worse than SHA256.
>
> — [github.com/BLAKE3-team/BLAKE3/issues/140](https://github.com/BLAKE3-team/BLAKE3/issues/140)

> Around 100,000 bytes, BLAKE3 is performing only twice as fast as SHA256
>
> — [github.com/BLAKE3-team/BLAKE3/issues/140](https://github.com/BLAKE3-team/BLAKE3/issues/140)

> An AMD Ryzen 9 3900X, so yeah, it has this SHA extension
>
> — [github.com/BLAKE3-team/BLAKE3/issues/140](https://github.com/BLAKE3-team/BLAKE3/issues/140)

> That microarchitecture supported AVX-512, which is very good for BLAKE3, but it didn't yet support the SHA extensions, which is of course bad for SHA-256.
>
> — [github.com/BLAKE3-team/BLAKE3/issues/140](https://github.com/BLAKE3-team/BLAKE3/issues/140)

> - `x86-sha`: uses the x86 SHA-NI extension.
>
> — [github.com/RustCrypto/hashes/blob/master/sha2/README.md](https://github.com/RustCrypto/hashes/blob/master/sha2/README.md)

</details>
