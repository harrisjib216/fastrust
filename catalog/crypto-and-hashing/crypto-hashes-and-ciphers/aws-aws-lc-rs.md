# aws-lc-rs (AWS-LC with a ring-compatible API)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Crypto and hashing](../README.md) › [Cryptographic hashes and ciphers](README.md)</sub>

**Tier B** (independent measurement) · ✅ Holds up · Open source · `Rust + C + assembly` · `ISC AND (Apache-2.0 OR ISC)` · 🟢 active

> Rust bindings to AWS-LC with ring's API: 3.2x RustCrypto's AES-GCM throughput and 10x its RSA-2048 signing rate on a Ryzen 9 7940HS.

| | |
|---|---|
| **Beats** | RustCrypto aes-gcm 0.11.1, p256 0.14.0, p384 0.14.0 and rsa 0.10.0-rc.19 on x86_64 |
| **Claimed result** | AES-256-GCM 8 KB: 9.48 GiB/s vs 2.92 GiB/s (3.2x, derived from page data); RSA-2048 signing: 5,487.4 vs 514.3 sigs/sec (10.7x, derived from page data); ECDSA P-384 signing: 16,583 vs 3,656.4 sigs/sec (4.5x, derived from page data); P-384 key agreement: 7,359.1 vs 795.15 kx/sec |
| **Workload** | graviola's criterion suite: AES-256-GCM encryption of 8 KB buffers, RSA-2048 and ECDSA P-256/P-384 signing and verification, X25519/P-256/P-384 key agreement, ML-KEM-768. |
| **Metric** | throughput |
| **Hardware** | Framework 16 laptop with AMD Ryzen 9 7940HS; MacBook Pro M2 (2022) |
| **OS** | Linux on the x86_64 laptop; macOS on the M2 |
| **Adoption** | 668 stars |
| **Last commit** | 2026-10-28 |
| **Platforms** | Linux, macOS, Windows and other targets AWS-LC supports; optional FIPS build |
| **Install** | cargo add aws-lc-rs |
| **Links** | [repository](https://github.com/aws/aws-lc-rs) · [documentation](https://docs.rs/aws-lc-rs) · [benchmark](https://jbp.io/graviola/) |

## Verdict

On the same independent benchmark (Ryzen 9 7940HS), aws-lc-rs encrypts AES-256-GCM at 9.48 GiB/s against 2.92 GiB/s for RustCrypto, signs RSA-2048 about 10.7x faster and ECDSA P-384 about 4.5x faster. It is first or second on every x86_64 row and well ahead of ring on P-384.

**Why tier B:** Third-party benchmark by graviola's author with CPU and crate versions stated; rustc version and flags are not shown.

## Use it when

- rustls or other TLS stacks where handshake and AEAD cost matter
- FIPS requirements (aws-lc-fips-sys)
- P-384 or RSA-heavy workloads

## Skip it when

- Pure-Rust or no-C-toolchain builds
- Targets AWS-LC does not support

## How it is fast

AWS-LC is a BoringSSL fork with hand-written assembly for AES-GCM, RSA and the NIST curves; aws-lc-rs calls it through pre-generated FFI bindings. It carries AVX-512 code paths (the benchmark page marks its x86_64 AES-GCM and RSA signing results as using AVX-512) and much faster P-384 code than ring, which is where it pulls ahead of ring.

## What it costs

- Needs a C/C++ compiler; FIPS builds also need CMake and Go
- Large C dependency compiled into the binary

## Problems with the evidence

- No run date or rustc version on the page (served Last-Modified 2026-10-27)
- The page marks the x86_64 AES-GCM and RSA signing results as using AVX-512; CPUs without it will see smaller gaps
- On the M2 run the RustCrypto AES-GCM figure (0.142 GiB/s vs 6.47 GiB/s) used aes-gcm 0.10.3, whose ARMv8 AES path needed opt-in flags, so that gap is overstated

## Evidence

<details>
<summary>16 quotes from the sources, checked 2026-10-28</summary>

> AES256-GCM encryption (8KB wide)
>
> — [jbp.io/graviola/](https://jbp.io/graviola/)

> 9.48 GiB/sec
>
> — [jbp.io/graviola/](https://jbp.io/graviola/)

> 2.92 GiB/sec
>
> — [jbp.io/graviola/](https://jbp.io/graviola/)

> RSA2048 signing
>
> — [jbp.io/graviola/](https://jbp.io/graviola/)

> 5,487.4 sigs/sec
>
> — [jbp.io/graviola/](https://jbp.io/graviola/)

> 514.3 sigs/sec
>
> — [jbp.io/graviola/](https://jbp.io/graviola/)

> 16,583 sigs/sec
>
> — [jbp.io/graviola/](https://jbp.io/graviola/)

> 3,656.4 sigs/sec
>
> — [jbp.io/graviola/](https://jbp.io/graviola/)

> 7,359.1 kx/sec
>
> — [jbp.io/graviola/](https://jbp.io/graviola/)

> 795.15 kx/sec
>
> — [jbp.io/graviola/](https://jbp.io/graviola/)

> 6.47 GiB/sec
>
> — [jbp.io/graviola/](https://jbp.io/graviola/)

> 0.142 GiB/sec
>
> — [jbp.io/graviola/](https://jbp.io/graviola/)

> Framework 16 with AMD Ryzen 9 7940HS CPU
>
> — [jbp.io/graviola/](https://jbp.io/graviola/)

> aws-lc-rs version: aws-lc-rs 1.18.2, aws-lc-sys 0.46.0
>
> — [jbp.io/graviola/](https://jbp.io/graviola/)

> rustcrypto version: aes-gcm 0.11.1, p256 0.14.0, p384 0.14.0, rsa 0.10.0-rc.19, sha2 0.11.1
>
> — [jbp.io/graviola/](https://jbp.io/graviola/)

> Consuming projects will need a C/C++ compiler to build.
>
> — [github.com/aws/aws-lc-rs/blob/main/aws-lc-rs/README.md](https://github.com/aws/aws-lc-rs/blob/main/aws-lc-rs/README.md)

</details>
