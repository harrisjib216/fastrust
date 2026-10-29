# base64 0.23 (Simd engine)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Data formats](../README.md) › [Base64, hex and encoding](README.md)</sub>

**Tier C** (first-party numbers) · ✅ Holds up · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> The base64 crate's own SIMD engine (0.23+): 3.7x faster encode and up to 7.1x faster decode than its scalar engine, if you opt in.

| | |
|---|---|
| **Beats** | base64's scalar GeneralPurpose engine (general_purpose::STANDARD, the only engine through 0.22 and still the prelude default in 0.23) |
| **Claimed result** | encode_slice 3 KiB: 1.73 us -> 468 ns (3.7x); 3 MiB: 1.74 ms -> 456 us (3.8x). decode_slice 3 KiB: 1.49 us -> 230 ns (6.5x); 3 MiB: 1.54 ms -> 216 us (7.1x). Instructions 22,245 -> 4,741 (4.7x) up to 28,705,264 -> 4,063,989 (7.1x). |
| **Workload** | encode_slice and decode_slice into existing buffers, 3 KiB and 3 MiB inputs, STANDARD alphabet. |
| **Metric** | time |
| **Hardware** | x86-64-v3 machine with AVX2 (model not stated) |
| **Measured** | 2026-07-19 |
| **Adoption** | 740 stars |
| **Last commit** | 2026-08-18 |
| **Platforms** | x86_64 AVX2 and aarch64 NEON; runtime detection (Simd) needs std; Avx2/Neon engines work in no_std; scalar fallback elsewhere |
| **Install** | cargo add base64 (0.23 or later), then use base64::engine::Simd |
| **Links** | [repository](https://github.com/marshallpierce/rust-base64) · [documentation](https://docs.rs/base64) · [benchmark](https://github.com/marshallpierce/rust-base64/pull/299) |

## Verdict

The default base64 crate gained SIMD engines in 0.23.0 (2026-07-23). Its PR measured 3.7x-3.8x faster encoding and 6.5x-7.1x faster decoding than the scalar engine on AVX2, with instruction counts down 4.7x-7.1x. The speedup is opt-in: STANDARD and the prelude constants are still scalar, so callers must construct engine::Simd.

**Why tier C:** First-party PR numbers from public criterion and Callgrind benchmarks; the machine is only described as AVX2, x86-64-v3.

## Use it when

- You already depend on base64 and encode or decode more than a few hundred bytes at a time
- x86_64 with AVX2 or any aarch64 target
- You want SIMD speed without adding base64-simd as another dependency

## Skip it when

- Custom alphabets: only STANDARD and URL_SAFE are accelerated
- Crates that must stay #![forbid(unsafe_code)]: disable the default simd-unsafe feature
- Secret material: the crate documents its GeneralPurpose engine as not constant-time and offers no constant-time SIMD engine

## How it is fast

Implements Wojciech Muła's vectorized base64 algorithms: AVX2 uses a multiply-based bit interleave, NEON a shift/mask variant, both with shared lookup tables. The Simd engine detects AVX2 or NEON at runtime and falls back to the scalar kernels for tails and unsupported CPUs.

## What it costs

- The simd-unsafe feature (on by default) is the crate's only use of unsafe
- MSRV rose to 1.71 in 0.23.0
- Opt-in API: existing code using STANDARD sees no change

## Problems with the evidence

- First-party numbers on an unnamed machine; no third-party run yet
- Only 3 KiB and 3 MiB inputs were measured; short inputs may gain less
- Wall-clock and instruction counts are from the PR description, not a published report

## Evidence

<details>
<summary>7 quotes from the sources, checked 2026-10-28</summary>

> | encode_slice | 3 KiB | 22,245 -> 4,741        (4.7x)  | 1.73 us -> 468 ns   (3.7x)  |
>
> — [api.github.com/repos/marshallpierce/rust-base64/pulls/299](https://api.github.com/repos/marshallpierce/rust-base64/pulls/299)

> | encode_slice | 3 MiB | 22,282,621 -> 3,802,013 (5.9x) | 1.74 ms -> 456 us   (3.8x)  |
>
> — [api.github.com/repos/marshallpierce/rust-base64/pulls/299](https://api.github.com/repos/marshallpierce/rust-base64/pulls/299)

> | decode_slice | 3 KiB | 28,620 -> 4,817        (5.9x)  | 1.49 us -> 230 ns   (6.5x)  |
>
> — [api.github.com/repos/marshallpierce/rust-base64/pulls/299](https://api.github.com/repos/marshallpierce/rust-base64/pulls/299)

> | decode_slice | 3 MiB | 28,705,264 -> 4,063,989 (7.1x) | 1.54 ms -> 216 us   (7.1x)  |
>
> — [api.github.com/repos/marshallpierce/rust-base64/pulls/299](https://api.github.com/repos/marshallpierce/rust-base64/pulls/299)

> times with the criterion benchmarks (AVX2, x86-64-v3, this machine):
>
> — [api.github.com/repos/marshallpierce/rust-base64/pulls/299](https://api.github.com/repos/marshallpierce/rust-base64/pulls/299)

> Added SIMD-accelerated engines behind the default-on `simd-unsafe` feature
>
> — [github.com/marshallpierce/rust-base64/blob/master/RELEASE-NOTES.md](https://github.com/marshallpierce/rust-base64/blob/master/RELEASE-NOTES.md)

> pub use crate::engine::general_purpose::STANDARD as BASE64_STANDARD;
>
> — [github.com/marshallpierce/rust-base64/blob/master/src/prelude.rs](https://github.com/marshallpierce/rust-base64/blob/master/src/prelude.rs)

</details>
