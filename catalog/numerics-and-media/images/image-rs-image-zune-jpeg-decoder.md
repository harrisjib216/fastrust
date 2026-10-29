# image 0.25: JPEG decoding switched to zune-jpeg

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Numerics and media](../README.md) › [Images](README.md)</sub>

**Tier C** (first-party numbers) · ✅ Holds up · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Since 0.25.0 the image crate decodes JPEG with zune-jpeg instead of jpeg-decoder; upgrading gets the 1.5-2x faster decoder for free.

| | |
|---|---|
| **Beats** | image 0.24 and earlier, which decoded JPEG with jpeg-decoder |
| **Claimed result** | zune-jpeg is 1.5x to 2x faster than jpeg-decoder (author figure), adopted as the default JPEG decoder in image Version 0.25.0 |
| **Workload** | JPEG decoding through image::open / ImageReader with the default jpeg feature. |
| **Metric** | time |
| **Measured** | 2023-03 |
| **Adoption** | 5.9k stars |
| **Last commit** | 2026-09-15 |
| **Platforms** | Any |
| **Install** | cargo add image (0.25.0 or later, released 2024-03-10) |
| **Links** | [repository](https://github.com/image-rs/image) · [documentation](https://docs.rs/image) · [benchmark](https://github.com/image-rs/image/blob/main/CHANGES.md) |

## Verdict

image 0.25.0 replaced jpeg-decoder with zune-jpeg, which its author measured 1.5x to 2x faster. Upgrading image from 0.24 to 0.25 or later is the cheapest way to get the faster decoder.

**Why tier C:** The switch is documented in image's release notes; the size of the gain rests on zune-jpeg's author's figure without workload detail.

## Use it when

- You are still on image 0.24 or earlier and decode JPEGs.

## Skip it when

- You need lossless JPEG decoding, which zune-jpeg does not support and jpeg-decoder does.

## How it is fast

image delegates JPEG decoding to its codec crate; 0.25.0 swapped jpeg-decoder for zune-jpeg, which has SIMD color conversion and a faster Huffman decoder. Later releases track zune-jpeg 0.5.

## What it costs

- Breaking API changes from 0.24 to 0.25; MSRV raised (1.88.0 for 0.25.10).

## Problems with the evidence

- No benchmark in image's own repository quantifies the switch; the figure is zune-jpeg's author's.
- image 0.25 also changed decoder traits (BufRead + Seek bounds, removed incremental decoding), so the upgrade is a breaking change.
- The tiff crate still pulled in jpeg-decoder for a while after the switch (image issue #2289).

## Evidence

<details>
<summary>4 quotes from the sources, checked 2026-10-28</summary>

> ### Version 0.25.0
>
> — [github.com/image-rs/image/blob/075e65d618a323e5cb48c1d8710a2eca25d6…](https://github.com/image-rs/image/blob/075e65d618a323e5cb48c1d8710a2eca25d690c6/CHANGES.md)

> Switched to zune-jpeg for JPEG decoding.
>
> — [github.com/image-rs/image/blob/075e65d618a323e5cb48c1d8710a2eca25d6…](https://github.com/image-rs/image/blob/075e65d618a323e5cb48c1d8710a2eca25d690c6/CHANGES.md)

> zune-jpeg = { version = "0.5.5", optional = true }
>
> — [github.com/image-rs/image/blob/v0.25.10/Cargo.toml](https://github.com/image-rs/image/blob/v0.25.10/Cargo.toml)

> zune-jpeg is 1.5x to 2x faster than jpeg-decoder and is on par with libjpeg-turbo
>
> — [libhunt.com/posts/1119512-announcing-zune-jpeg-rust-s-fastest-jpeg-…](https://www.libhunt.com/posts/1119512-announcing-zune-jpeg-rust-s-fastest-jpeg-decoder)

</details>
