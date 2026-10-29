# RustCrypto asm-hashes (the sha2 'asm' feature)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Crypto and hashing](../README.md) › [Cryptographic hashes and ciphers](README.md)</sub>

**Tier D** (directional only) · ❌ Doesn't hold up · Open source · `Rust + assembly` · `MIT` · 🔴 abandoned

> Assembly SHA-1/SHA-2/MD5 compression functions for RustCrypto; archived and dropped from sha2 0.11 with no published speedup.

| | |
|---|---|
| **Beats** | sha2's built-in backends: portable Rust plus SHA-NI and ARMv8 SHA2 intrinsics selected at runtime |
| **Claimed result** | No numbers published; the maintainers say the asm backends 'can be a bit faster than the software backends'. |
| **Workload** | Not measured |
| **Metric** | throughput |
| **Adoption** | 55 stars |
| **Last commit** | 2024-05-07 |
| **Platforms** | x86 and x86_64 Linux, ARM64; Windows x86_64 GNU only (MSVC builds known broken) |
| **Install** | sha2 0.10 with features = ["asm"] (removed in sha2 0.11) |
| **Links** | [repository](https://github.com/RustCrypto/asm-hashes) · [documentation](https://docs.rs/sha2-asm) · [benchmark](https://github.com/RustCrypto/hashes/pull/542) |

## Verdict

The asm feature was removed in sha2 0.11.0 and the asm-hashes repository is archived. Its maintainers only ever described it as 'a bit faster' than the software backend, and sha2 already uses SHA-NI and ARMv8 SHA2 instructions at runtime, which is where large gains come from.

**Why tier D:** No published numbers, only a maintainer's qualitative statement.

## Use it when

- Only if you are pinned to sha2 0.10 on a CPU without SHA extensions and have measured a gain yourself

## Skip it when

- sha2 0.11 or later: the feature no longer exists
- Windows MSVC: builds are known to be broken
- CPUs with SHA-NI or ARMv8 SHA2, where sha2 already uses the hardware instructions

## How it is fast

Hand-written assembly compression functions based on Project Nayuki's code, built with cc, replaced the portable Rust compression loop when the asm feature was enabled.

## What it costs

- Depends on cc and a C toolchain, with limited cross-platform support per the maintainers

## Problems with the evidence

- No benchmark numbers were found for the asm backends

## Evidence

<details>
<summary>6 quotes from the sources, checked 2026-10-28</summary>

> The asm-hashes backends can be a bit faster than the software backends, but they are a maintenance headache
>
> — [github.com/RustCrypto/hashes/pull/542](https://github.com/RustCrypto/hashes/pull/542)

> ## 0.11.0 (2026-03-25)
>
> — [github.com/RustCrypto/hashes/blob/master/sha2/CHANGELOG.md](https://github.com/RustCrypto/hashes/blob/master/sha2/CHANGELOG.md)

> - `asm`, `asm-aarch64`, `loongarch64_asm`, and `compress` crate features ([#542])
>
> — [github.com/RustCrypto/hashes/blob/master/sha2/CHANGELOG.md](https://github.com/RustCrypto/hashes/blob/master/sha2/CHANGELOG.md)

> ## NOTE: this repo is in maintenance mode only
>
> — [github.com/RustCrypto/asm-hashes/blob/master/README.md](https://github.com/RustCrypto/asm-hashes/blob/master/README.md)

> Windows MSVC builds are known to be broken.
>
> — [github.com/RustCrypto/asm-hashes/blob/master/README.md](https://github.com/RustCrypto/asm-hashes/blob/master/README.md)

> - `x86-sha`: uses the x86 SHA-NI extension.
>
> — [github.com/RustCrypto/hashes/blob/master/sha2/README.md](https://github.com/RustCrypto/hashes/blob/master/sha2/README.md)

</details>
