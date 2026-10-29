# crc-fast (SIMD CRC-16/32/64 for all catalogued variants)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Crypto and hashing](../README.md) › [Checksums and fast hashes](README.md)</sub>

**Tier C** (first-party numbers) · 🟡 Holds up in part · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> SIMD CRC for every CRC-16/32/64 variant, reaching ~111 GiB/s on AVX-512 servers; the '>220X' headline compares against crc's slowest default.

| | |
|---|---|
| **Beats** | crc crate 3.x with its default bytewise table (about 0.5 GiB/s) |
| **Claimed result** | Up to >220X faster than the crc crate at ~0.5GiB/s; CRC-32/ISCSI ~111 GiB/s (1 MiB) and ~61 GiB/s (1 KiB) on Sapphire Rapids with avx512-vpclmulqdq; CRC-32/ISO-HDLC ~88 GiB/s (1 MiB); Graviton2 ~17 GiB/s (1 MiB); crc's Slice16 is 3.30 GiB/s on a Ryzen 7 3800X. |
| **Workload** | CRC-32/ISCSI, CRC-32/ISO-HDLC, CRC-64/NVME and forward variants over 1 KiB and 1 MiB buffers. |
| **Metric** | throughput |
| **Hardware** | Intel Sapphire Rapids (EC2 c7i.metal-24xl), AMD Genoa (EC2 c7a.metal-48xl), AWS Graviton4 and Graviton2, Apple M3 Ultra, Apple M4 Max |
| **Adoption** | 95 stars |
| **Last commit** | 2025-12-27 |
| **Platforms** | aarch64, x86_64 and x86 SIMD; table-based fallback elsewhere; C-compatible library |
| **Install** | cargo add crc-fast |
| **Links** | [repository](https://github.com/awesomized/crc-fast-rust) · [documentation](https://docs.rs/crc-fast) · [benchmark](https://github.com/awesomized/crc-fast-rust/blob/main/README.md) |

## Verdict

The '>220X' headline pairs the best row (about 111 GiB/s for CRC-32/ISCSI on 1 MiB inputs with AVX-512 VPCLMULQDQ on Sapphire Rapids) with the crc crate's default bytewise table measured elsewhere. On 1 KiB inputs the same machine shows about 61 GiB/s, Graviton2 reaches about 17 GiB/s at 1 MiB, and crc's own Slice16 option does 3.30 GiB/s, so against a tuned software CRC the gain is roughly 5x to 34x (derived from page data), not 220x.

**Why tier C:** CPU and SIMD target are stated per row, but the 220x comparison uses a crc figure from another machine, values are rounded, and no rustc version or command is given.

## Use it when

- CRC-64/NVME (S3 checksums), CRC-32C or uncommon CRC variants that crc32fast does not cover
- Large buffers on AVX-512 VPCLMULQDQ servers or recent ARM cores

## Skip it when

- Toolchains older than 1.89 (crate MSRV)
- Small inputs where call overhead dominates

## How it is fast

It generalizes Intel's PCLMULQDQ folding method to any Rocksoft-model polynomial, folding 1 MiB inputs with VPCLMULQDQ on 512-bit registers or PMULL on ARM. For CRC-32/ISCSI and, on aarch64, CRC-32/ISO-HDLC it fuses carry-less multiply folding with the CPU's native CRC instructions.

## What it costs

- MSRV 1.89
- unsafe SIMD intrinsics

## Problems with the evidence

- Throughput values are rounded approximations (~)
- x86 rows are all AVX-512 VPCLMULQDQ server CPUs; older x86 without VPCLMULQDQ is not shown
- No head-to-head with crc32fast or crc32c on the same machine

## Evidence

<details>
<summary>7 quotes from the sources, checked 2026-10-28</summary>

> is ~0.5GiB/s by default, so this is
>
> — [github.com/awesomized/crc-fast-rust/blob/main/README.md](https://github.com/awesomized/crc-fast-rust/blob/main/README.md)

> up to >220X faster
>
> — [github.com/awesomized/crc-fast-rust/blob/main/README.md](https://github.com/awesomized/crc-fast-rust/blob/main/README.md)

> | x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-24xl        | avx512-vpclmulqdq |          ~61 |         ~111 |
>
> — [github.com/awesomized/crc-fast-rust/blob/main/README.md](https://github.com/awesomized/crc-fast-rust/blob/main/README.md)

> | x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-248xl       | avx512-vpclmulqdq |          ~28 |          ~88 |
>
> — [github.com/awesomized/crc-fast-rust/blob/main/README.md](https://github.com/awesomized/crc-fast-rust/blob/main/README.md)

> | aarch64 | AWS   | Graviton2       | EC2 c6g.metal             | neon-pmull        |          ~11 |          ~17 |
>
> — [github.com/awesomized/crc-fast-rust/blob/main/README.md](https://github.com/awesomized/crc-fast-rust/blob/main/README.md)

> | 32    | 0.111   | 0.516    | 3.30    |
>
> — [github.com/mrhooray/crc-rs/blob/master/README.md](https://github.com/mrhooray/crc-rs/blob/master/README.md)

> with AMD Ryzen 7 3800X
>
> — [github.com/mrhooray/crc-rs/blob/master/README.md](https://github.com/mrhooray/crc-rs/blob/master/README.md)

</details>
