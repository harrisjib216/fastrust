# ring (BoringSSL-derived crypto primitives)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Crypto and hashing](../README.md) › [Cryptographic hashes and ciphers](README.md)</sub>

**Tier B** (independent measurement) · ✅ Holds up · Open source · `Rust + C + assembly` · `Apache-2.0 AND ISC` · 🟢 active

> Rust API over BoringSSL's assembly: about 3x RustCrypto's AES-GCM throughput and 8x its ECDSA P-256 signing rate on a Ryzen 9 7940HS.

| | |
|---|---|
| **Beats** | RustCrypto aes-gcm 0.11.1, p256 0.14.0 and rsa 0.10.0-rc.19 on x86_64; aes-gcm 0.10.3 on Apple M2 |
| **Claimed result** | Ryzen 9 7940HS: AES-256-GCM 8 KB 8.78 GiB/s vs 2.92 GiB/s (3.0x, derived from page data); ECDSA P-256 signing 81,642 vs 9,350.8 sigs/sec (8.7x, derived from page data); P-256 key agreement 24,490 vs 2,100.9 kx/sec. Apple M2, AES-256-GCM 1200 B seal+open x 100k: 74.2ms vs 157ms (aes-gcm with ARMv8 flags) and 1.87s (aes-gcm default build). |
| **Workload** | graviola's criterion suite: AES-256-GCM encryption of 8 KB buffers, RSA-2048 and ECDSA P-256/P-384 signing and verification, X25519/P-256/P-384 key agreement. webrtc-rs: AES-256-GCM seal+open of a 1200-byte packet with a 12-byte AAD, 100,000 iterations. |
| **Metric** | throughput |
| **Hardware** | Framework 16 laptop with AMD Ryzen 9 7940HS; MacBook Pro M2 (2022); Apple M2 (webrtc-rs) |
| **OS** | Linux on the x86_64 laptop; macOS on the M2 |
| **Adoption** | 4.1k stars |
| **Last commit** | 2026-07-22 |
| **Platforms** | Targets BoringSSL's C and assembly support (x86_64, aarch64 and others); needs a C compiler |
| **Install** | cargo add ring |
| **Links** | [repository](https://github.com/briansmith/ring) · [documentation](https://docs.rs/ring) · [benchmark](https://jbp.io/graviola/) |

## Verdict

An independent benchmark on a Ryzen 9 7940HS puts ring at 8.78 GiB/s for AES-256-GCM against 2.92 GiB/s for RustCrypto's aes-gcm, and far ahead on ECDSA P-256. A separate webrtc-rs A/B on an Apple M2 found ring 2.7x fewer cycles per packet than aes-gcm with its ARMv8 backend switched on. The 38x AES-GCM gap on the graviola M2 run is overstated: it used aes-gcm 0.10.3, whose ARMv8 AES path needed opt-in cfg flags, while the current aes crate detects ARMv8 AES at runtime on Linux and macOS.

**Why tier B:** Third-party benchmarks (graviola's author and the webrtc-rs project) with CPUs and crate versions stated; rustc version and flags are not shown for graviola.

## Use it when

- TLS or bulk AEAD where AES-GCM throughput matters
- High-volume ECDSA P-256 or RSA signing and verification

## Skip it when

- Pure-Rust builds without a C toolchain, or targets BoringSSL does not support
- P-384-heavy workloads: aws-lc-rs signed 16,583 vs 3,358.7 per second on the same x86_64 laptop

## How it is fast

The hot paths are BoringSSL's hand-tuned assembly: AES-GCM runs as one stitched pass that interleaves AES-NI/VAES encryption with carry-less-multiply GHASH, and the P-256 code uses dedicated field arithmetic. RustCrypto's aes-gcm makes separate AES and GHASH passes, which the webrtc-rs authors cite for the remaining 2.7x on ARM.

## What it costs

- C and assembly from BoringSSL: needs a C compiler and inherits its side-channel limits
- The README calls the project an experiment and repeats BoringSSL's warning against general use
- Narrower algorithm set than RustCrypto

## Problems with the evidence

- The graviola page has no run date (served Last-Modified 2026-10-27) and no rustc version; benchmarking is described as non-automated on two laptops
- The page marks ring's and aws-lc-rs's x86_64 AES-GCM results as using AVX-512; CPUs without AVX-512/VAES will see a smaller gap
- The M2 rows use older RustCrypto releases (aes-gcm 0.10.3) than the x86_64 rows

## Evidence

<details>
<summary>18 quotes from the sources, checked 2026-10-28</summary>

> AES256-GCM encryption (8KB wide)
>
> — [jbp.io/graviola/](https://jbp.io/graviola/)

> 8.78 GiB/sec
>
> — [jbp.io/graviola/](https://jbp.io/graviola/)

> 2.92 GiB/sec
>
> — [jbp.io/graviola/](https://jbp.io/graviola/)

> 81,642 sigs/sec
>
> — [jbp.io/graviola/](https://jbp.io/graviola/)

> 9,350.8 sigs/sec
>
> — [jbp.io/graviola/](https://jbp.io/graviola/)

> 24,490 kx/sec
>
> — [jbp.io/graviola/](https://jbp.io/graviola/)

> 2,100.9 kx/sec
>
> — [jbp.io/graviola/](https://jbp.io/graviola/)

> Framework 16 with AMD Ryzen 9 7940HS CPU
>
> — [jbp.io/graviola/](https://jbp.io/graviola/)

> rustcrypto version: aes-gcm 0.11.1, p256 0.14.0, p384 0.14.0, rsa 0.10.0-rc.19, sha2 0.11.1
>
> — [jbp.io/graviola/](https://jbp.io/graviola/)

> rustcrypto version: aes-gcm 0.10.3, p256 0.13.2, p384 0.13.1, rsa 0.9.10, sha2 0.10.9
>
> — [jbp.io/graviola/](https://jbp.io/graviola/)

> At the moment benchmarking is non-automated, and is run on two of my laptops.
>
> — [jbp.io/graviola/](https://jbp.io/graviola/)

> ## Results (Apple M2, AES-256-GCM, 1200 B seal+open, 100k iters)
>
> — [github.com/webrtc-rs/rtc/blob/master/benchmarks/aead-gcm/README.md](https://github.com/webrtc-rs/rtc/blob/master/benchmarks/aead-gcm/README.md)

> wall_time          1.87s  ± 20.6ms
>
> — [github.com/webrtc-rs/rtc/blob/master/benchmarks/aead-gcm/README.md](https://github.com/webrtc-rs/rtc/blob/master/benchmarks/aead-gcm/README.md)

> wall_time           157ms ± 3.04ms
>
> — [github.com/webrtc-rs/rtc/blob/master/benchmarks/aead-gcm/README.md](https://github.com/webrtc-rs/rtc/blob/master/benchmarks/aead-gcm/README.md)

> wall_time          74.2ms ± 5.40ms
>
> — [github.com/webrtc-rs/rtc/blob/master/benchmarks/aead-gcm/README.md](https://github.com/webrtc-rs/rtc/blob/master/benchmarks/aead-gcm/README.md)

> ~2.7x fewer than even hardware-flagged RustCrypto (ring's single pass vs `aes-gcm`'s two)
>
> — [github.com/webrtc-rs/rtc/blob/master/benchmarks/aead-gcm/README.md](https://github.com/webrtc-rs/rtc/blob/master/benchmarks/aead-gcm/README.md)

> $ RUSTFLAGS="--cfg aes_armv8 --cfg polyval_armv8"
>
> — [github.com/webrtc-rs/rtc/blob/master/benchmarks/aead-gcm/README.md](https://github.com/webrtc-rs/rtc/blob/master/benchmarks/aead-gcm/README.md)

> On Linux and macOS, support for ARMv8 AES intrinsics is autodetected at
>
> — [github.com/RustCrypto/block-ciphers/blob/master/aes/src/lib.rs](https://github.com/RustCrypto/block-ciphers/blob/master/aes/src/lib.rs)

</details>
