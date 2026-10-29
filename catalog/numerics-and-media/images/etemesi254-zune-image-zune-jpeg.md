# zune-jpeg (JPEG decoder)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Numerics and media](../README.md) › [Images](README.md)</sub>

**Tier C** (first-party numbers) · ❌ Doesn't hold up · Open source · `Rust` · `MIT OR Apache-2.0 OR Zlib` · 🟢 active

> Pure-Rust JPEG decoder, 1.5-2x faster than jpeg-decoder per its author; it is now the image crate's default JPEG decoder.

| | |
|---|---|
| **Beats** | jpeg-decoder 0.3, the image crate's JPEG decoder before image 0.25 |
| **Claimed result** | zune-jpeg is 1.5x to 2x faster than jpeg-decoder and is on par with libjpeg-turbo (author, March 2023). Current first-party run on Darwin, baseline decode: mozjpeg 50.00 ms vs zune-jpeg 54.83 ms (1.097x slower) |
| **Workload** | Not stated for the 1.5x-2x figure. Current zune-image benchmarks: decoding baseline, progressive and subsampled JPEGs, time per image. |
| **Metric** | time |
| **OS** | Current run: Darwin (macOS) |
| **Measured** | 2023-03 |
| **Adoption** | 505 stars |
| **Last commit** | 2026-10-24 |
| **Platforms** | Any; AVX2 and SSE intrinsics on x86_64, scalar path elsewhere |
| **Install** | cargo add zune-jpeg (or use image 0.25, which uses it by default) |
| **Links** | [repository](https://github.com/etemesi254/zune-image) · [documentation](https://docs.rs/zune-jpeg) · [benchmark](https://github.com/etemesi254/zune-image/blob/dev/benchmarks/benchmark-results.md) |

## Verdict

The 1.5x-2x lead over jpeg-decoder was real enough that the image crate adopted zune-jpeg as its JPEG decoder in 0.25.0 (March 2024). Anyone decoding JPEG through image 0.25 already uses zune-jpeg, so adding it directly no longer beats the default.

**Why tier C:** The speedup is the author's own announcement figure with no workload or hardware attached, available via a mirror of the r/rust post.

## Use it when

- You want JPEG decoding without the image crate's abstraction, for example to control decoder options or avoid image's dependency tree.

## Skip it when

- You already decode through image 0.25 or later; it is the same decoder.
- You need lossless JPEG, which jpeg-decoder still supports.

## How it is fast

zune-jpeg combines a fast Huffman decoder, an 8x8 integer IDCT and fast color conversion, using AVX2 and SSE intrinsics on x86_64 for the heavy loops.

## What it costs

- Not bit-identical to libjpeg output, by design.
- Uses unsafe platform intrinsics for SIMD.

## Problems with the evidence

- The 1.5x-2x figure has no published workload, hardware or version, and the original r/rust post is quoted here through a mirror.
- The README says zune-jpeg should replace libjpeg-turbo "without any noticeable speed loss", but the author's 2026 Darwin run has it 1.10x to 1.34x slower than mozjpeg (libjpeg-turbo based) on baseline and subsampled images, and slightly faster on progressive ones.
- The current benchmarks no longer include jpeg-decoder, so there is no recent head-to-head.

## Evidence

<details>
<summary>7 quotes from the sources, checked 2026-10-28</summary>

> zune-jpeg is 1.5x to 2x faster than jpeg-decoder and is on par with libjpeg-turbo
>
> — [libhunt.com/posts/1119512-announcing-zune-jpeg-rust-s-fastest-jpeg-…](https://www.libhunt.com/posts/1119512-announcing-zune-jpeg-rust-s-fastest-jpeg-decoder)

> Post date: 1 Mar 2023
>
> — [libhunt.com/posts/1119512-announcing-zune-jpeg-rust-s-fastest-jpeg-…](https://www.libhunt.com/posts/1119512-announcing-zune-jpeg-rust-s-fastest-jpeg-decoder)

> | **No sampling Baseline decode**         | mozjpeg                      | 50.00 ms    | 24.30 MiB/s       | **Winner**        |
>
> — [github.com/etemesi254/zune-image/blob/2f9086b07cdf5a36621ce18f79d31…](https://github.com/etemesi254/zune-image/blob/2f9086b07cdf5a36621ce18f79d310f99b751de9/benchmarks/benchmark-results.md)

> |                                         | zune-jpeg                    | 54.83 ms    | 22.16 MiB/s       | 1.097× slower     |
>
> — [github.com/etemesi254/zune-image/blob/2f9086b07cdf5a36621ce18f79d31…](https://github.com/etemesi254/zune-image/blob/2f9086b07cdf5a36621ce18f79d310f99b751de9/benchmarks/benchmark-results.md)

> Switched to zune-jpeg for JPEG decoding.
>
> — [github.com/image-rs/image/blob/075e65d618a323e5cb48c1d8710a2eca25d6…](https://github.com/image-rs/image/blob/075e65d618a323e5cb48c1d8710a2eca25d690c6/CHANGES.md)

> This crate is in maintenance mode as _image-rs_ seeks to move over to the zune-jpeg crate.
>
> — [github.com/image-rs/jpeg-decoder](https://github.com/image-rs/jpeg-decoder)

> `zune-jpeg` should easily replace `libjpeg-turbo` without any noticeable speed loss.
>
> — [github.com/etemesi254/zune-image](https://github.com/etemesi254/zune-image)

</details>
