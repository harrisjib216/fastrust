# fast_image_resize (SIMD image resizing)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Numerics and media](../README.md) › [Images](README.md)</sub>

**Tier A** (reproducible public benchmark) · ✅ Holds up · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> SIMD image resizer that downsizes 5-27x faster than the image crate's resize, depending on filter, pixel format and CPU.

| | |
|---|---|
| **Beats** | image 0.25.6 resize (imageops) on the same images and filters |
| **Claimed result** | RGB8 4928x3279 -> 852x567, ms (image vs fir avx2 / fir sse4.1 / fir rust): Lanczos3 176.09 vs 13.52 / 15.91 / 37.26; Bilinear 80.24 vs 3.55; L8 Lanczos3 116.76 vs 4.26 (13.0x, 11.1x, 4.7x; 22.6x; 27.4x) (derived from page data). Arm Neoverse-N1 RGB8 Lanczos3: image 393.06 vs fir neon 79.72 ms (4.9x) (derived from page data). zune-image benchmark, Lanczos resize: image-rs 724.4 ms vs fir 27.89 ms (26x) (derived from page data) |
| **Workload** | Downscaling a 4928x3279 photo to 852x567, single-threaded, with nearest, box, bilinear, bicubic and Lanczos3 filters on RGB8, RGBA8 (with alpha premultiplication) and L8 pixels; time per resize. |
| **Metric** | time |
| **Hardware** | AMD Ryzen 9 5950X, DDR4-4000 (x86_64); Neoverse-N1 2GHz, Oracle Cloud VM.Standard.A1.Flex (ARM64) |
| **OS** | Ubuntu 24.04 |
| **Toolchain** | Rust 1.97.1, criterion 0.7.0, fast_image_resize 6.0.1; image 0.25.6, resize 0.8.9, libvips 8.15.1 single-threaded |
| **Measured** | 2026-07 |
| **Adoption** | 465 stars |
| **Last commit** | 2026-08-20 |
| **Platforms** | x86_64 (SSE4.1, AVX2 with runtime detection), aarch64 NEON, wasm32 simd128, scalar fallback; no_std feature |
| **Install** | cargo add fast_image_resize |
| **Links** | [repository](https://github.com/Cykooz/fast_image_resize) · [documentation](https://docs.rs/fast_image_resize) · [benchmark](https://github.com/Cykooz/fast_image_resize/blob/main/benchmarks-x86_64.md) |

## Verdict

On x86_64 it resizes 13x faster than the image crate with Lanczos3 and 22.6x faster with bilinear using AVX2, and still 4.7x faster with SIMD disabled. A separate benchmark by the zune-image author shows the same order of gap (26x on Lanczos). The image crate's unreleased next version switches to a faster resizer, which will narrow this.

**Why tier A:** Public criterion benchmarks with CPU, OS, rustc version and every compared crate version stated, plus a third-party run in zune-image's benchmarks.

## Use it when

- Thumbnail and image-processing services that resize many photos, especially with Lanczos or bicubic filters.
- You need correct alpha handling: it premultiplies and un-premultiplies alpha, which image 0.25 does not.

## Skip it when

- You need gamma-correct resizing without extra steps: it does not convert to linear light by itself (it provides mappers for that).
- Resizing is a negligible part of your pipeline compared with decoding and encoding.

## How it is fast

Convolution coefficients are precomputed once per row and column and applied in fixed-point integer arithmetic with SSE4.1, AVX2, NEON or simd128 kernels chosen at runtime. Resizing is done as two separable passes over contiguous rows, and alpha premultiplication is also vectorized.

## What it costs

- Uses unsafe SIMD intrinsics internally.
- Its own image types; converting from image::DynamicImage needs the image feature or manual buffer handling.
- No colorspace conversion by default.

## Problems with the evidence

- Benchmarks are by the crate's author; the zune-image run is independent but on an unspecified Darwin machine.
- The image crate's main branch has replaced DynamicImage resizing with the pic-scale-safe crate, which its PR describes as about 5x faster single-threaded; it is not in any release as of image 0.25.10, but once released the gap will shrink.
- Speedups are largest with AVX2; on the ARM64 VM the gap over image is 4.9x.

## Evidence

<details>
<summary>18 quotes from the sources, checked 2026-10-28</summary>

> | image      |  30.36  |   -   |  80.24   | 135.29  |  176.09  |
>
> — [github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff…](https://github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff4caff7cf163e19/benchmarks-x86_64.md)

> | fir avx2   |  0.28   | 2.65  |   3.55   |  6.90   |  13.52   |
>
> — [github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff…](https://github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff4caff7cf163e19/benchmarks-x86_64.md)

> | fir sse4.1 |  0.28   | 3.82  |   5.83   |  10.34  |  15.91   |
>
> — [github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff…](https://github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff4caff7cf163e19/benchmarks-x86_64.md)

> | fir rust   |  0.28   | 10.75 |  15.62   |  26.21  |  37.26   |
>
> — [github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff…](https://github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff4caff7cf163e19/benchmarks-x86_64.md)

> | image      |  28.88  |   -   |  60.77   |  89.05  |  116.76  |
>
> — [github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff…](https://github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff4caff7cf163e19/benchmarks-x86_64.md)

> | fir avx2   |  0.15   | 1.82  |   1.84   |  2.84   |   4.26   |
>
> — [github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff…](https://github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff4caff7cf163e19/benchmarks-x86_64.md)

> Resize RGB8 image (U8x3) 4928x3279 => 852x567
>
> — [github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff…](https://github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff4caff7cf163e19/benchmarks-x86_64.md)

> Numbers in the table mean the duration of the image resizing in milliseconds.
>
> — [github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff…](https://github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff4caff7cf163e19/benchmarks-x86_64.md)

> CPU: AMD Ryzen 9 5950X
>
> — [github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff…](https://github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff4caff7cf163e19/benchmarks-x86_64.md)

> Rust 1.97.1
>
> — [github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff…](https://github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff4caff7cf163e19/benchmarks-x86_64.md)

> image = "0.25.6"
>
> — [github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff…](https://github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff4caff7cf163e19/benchmarks-x86_64.md)

> The `image` crate does not support multiplying and dividing by alpha channel.
>
> — [github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff…](https://github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff4caff7cf163e19/benchmarks-x86_64.md)

> | image    |  83.47  |   -    |  161.06  | 279.08  |  393.06  |
>
> — [github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff…](https://github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff4caff7cf163e19/benchmarks-arm64.md)

> | fir neon |  0.89   | 18.98  |  29.21   |  53.59  |  79.72   |
>
> — [github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff…](https://github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff4caff7cf163e19/benchmarks-arm64.md)

> CPU: Neoverse-N1 2GHz (Oracle Cloud Compute, VM.Standard.A1.Flex)
>
> — [github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff…](https://github.com/Cykooz/fast_image_resize/blob/ce36aeb20ed4a33805baa3c0ff4caff7cf163e19/benchmarks-arm64.md)

> | **resize - lanczos-kernel**                | fir                                     | 27.89 ms    | 43.57 MiB/s       | 1.88× slower      |
>
> — [github.com/etemesi254/zune-image/blob/2f9086b07cdf5a36621ce18f79d31…](https://github.com/etemesi254/zune-image/blob/2f9086b07cdf5a36621ce18f79d310f99b751de9/benchmarks/benchmark-results.md)

> |                                            | image-rs                                | 724.4 ms    | 1.68 MiB/s        | 48.84× slower     |
>
> — [github.com/etemesi254/zune-image/blob/2f9086b07cdf5a36621ce18f79d31…](https://github.com/etemesi254/zune-image/blob/2f9086b07cdf5a36621ce18f79d310f99b751de9/benchmarks/benchmark-results.md)

> This PR migrates resizing DynamicImage from the built-in resizing impl
>
> — [github.com/image-rs/image/pull/2639](https://github.com/image-rs/image/pull/2639)

</details>
