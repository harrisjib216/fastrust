# zune-png (PNG decoder)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Numerics and media](../README.md) › [Images](README.md)</sub>

**Tier C** (first-party numbers) · 🟡 Holds up in part · Open source · `Rust` · `MIT OR Apache-2.0 OR Zlib` · 🟢 active

> Pure-Rust PNG/APNG decoder; faster than the png crate on 8-bit and interlaced images, slower on 16-bit and palette images.

| | |
|---|---|
| **Beats** | png 0.18 (image-rs/png), the image crate's PNG decoder |
| **Claimed result** | Decode time, image-rs/png vs zune-png: baseline 149.3 ms vs 135.0 ms, interlaced 8bpp 232.8 ms vs 139.7 ms (zune-png 1.11x and 1.67x faster); 16 bpp 163.7 ms vs 186.3 ms, palette 28.31 ms vs 35.21 ms (png 1.14x and 1.24x faster) |
| **Workload** | Decoding PNG files: 8-bit baseline, 8-bit Adam7 interlaced, 16-bit and palette images; criterion time per image. |
| **Metric** | time |
| **Hardware** | Not stated (Darwin) |
| **OS** | Darwin (macOS) |
| **Measured** | 2026-09-21 |
| **Adoption** | 505 stars |
| **Last commit** | 2026-10-24 |
| **Platforms** | Any; SSE and NEON intrinsics for unfiltering, scalar fallback |
| **Install** | cargo add zune-png |
| **Links** | [repository](https://github.com/etemesi254/zune-image) · [documentation](https://docs.rs/zune-png) · [benchmark](https://github.com/etemesi254/zune-image/blob/dev/benchmarks/benchmark-results.md) |

## Verdict

In its author's current run zune-png is 1.11x faster than the png crate on a baseline 8-bit image and 1.67x faster on interlaced images, but png is faster on 16-bit images (1.14x) and palette images (1.24x). The default has caught up enough that the choice depends on the image type.

**Why tier C:** First-party criterion results; the benchmark code is public but the machine is identified only as Darwin, with no CPU or rustc version.

## Use it when

- You decode many interlaced or 8-bit RGB/RGBA PNGs and PNG decoding shows up in profiles.

## Skip it when

- Your images are mostly 16-bit or palette-based; the png crate was faster there.
- You decode through the image crate and do not want a second PNG decoder.

## How it is fast

zune-png pairs its own inflate implementation (zune-inflate) with SSE and NEON intrinsics for PNG unfiltering, the two stages that dominate PNG decode time.

## What it costs

- Expands images below 8 bits per pixel to 8 bits automatically.
- Smaller user base than the png crate, which the image crate uses.

## Problems with the evidence

- Single first-party run on an unspecified Darwin machine; no x86_64 numbers are published alongside.
- The png crate has itself become much faster: in the same run it decodes 2.0x to 3.3x faster than spng, while Google's 2022 benchmark had png slower than spng on large images.
- The zune-image README says its decoders are "some of the fastest" without per-format qualifiers.

## Evidence

<details>
<summary>12 quotes from the sources, checked 2026-10-28</summary>

> | **PNG decoding baseline**        | image-rs/png | 149.3 ms    | 40.12 MiB/s       | 1.11× slower      |
>
> — [github.com/etemesi254/zune-image/blob/2f9086b07cdf5a36621ce18f79d31…](https://github.com/etemesi254/zune-image/blob/2f9086b07cdf5a36621ce18f79d310f99b751de9/benchmarks/benchmark-results.md)

> |                                  | zune-png     | 135.0 ms    | 44.38 MiB/s       | **Winner**        |
>
> — [github.com/etemesi254/zune-image/blob/2f9086b07cdf5a36621ce18f79d31…](https://github.com/etemesi254/zune-image/blob/2f9086b07cdf5a36621ce18f79d310f99b751de9/benchmarks/benchmark-results.md)

> | **PNG decoding interlaced 8bpp** | image-rs/png | 232.8 ms    | 43.33 MiB/s       | 1.67× slower      |
>
> — [github.com/etemesi254/zune-image/blob/2f9086b07cdf5a36621ce18f79d31…](https://github.com/etemesi254/zune-image/blob/2f9086b07cdf5a36621ce18f79d310f99b751de9/benchmarks/benchmark-results.md)

> |                                  | zune-png     | 139.7 ms    | 72.22 MiB/s       | **Winner**        |
>
> — [github.com/etemesi254/zune-image/blob/2f9086b07cdf5a36621ce18f79d31…](https://github.com/etemesi254/zune-image/blob/2f9086b07cdf5a36621ce18f79d310f99b751de9/benchmarks/benchmark-results.md)

> | **PNG decoding  16 bpp**         | image-rs/png | 163.7 ms    | 46.92 MiB/s       | **Winner**        |
>
> — [github.com/etemesi254/zune-image/blob/2f9086b07cdf5a36621ce18f79d31…](https://github.com/etemesi254/zune-image/blob/2f9086b07cdf5a36621ce18f79d310f99b751de9/benchmarks/benchmark-results.md)

> |                                  | zune-png     | 186.3 ms    | 41.23 MiB/s       | 1.14× slower      |
>
> — [github.com/etemesi254/zune-image/blob/2f9086b07cdf5a36621ce18f79d31…](https://github.com/etemesi254/zune-image/blob/2f9086b07cdf5a36621ce18f79d310f99b751de9/benchmarks/benchmark-results.md)

> | **PNG decoding palette image**   | image-rs/png | 28.31 ms    | 184.1 MiB/s       | **Winner**        |
>
> — [github.com/etemesi254/zune-image/blob/2f9086b07cdf5a36621ce18f79d31…](https://github.com/etemesi254/zune-image/blob/2f9086b07cdf5a36621ce18f79d310f99b751de9/benchmarks/benchmark-results.md)

> |                                  | zune-png     | 35.21 ms    | 148.0 MiB/s       | 1.24× slower      |
>
> — [github.com/etemesi254/zune-image/blob/2f9086b07cdf5a36621ce18f79d31…](https://github.com/etemesi254/zune-image/blob/2f9086b07cdf5a36621ce18f79d310f99b751de9/benchmarks/benchmark-results.md)

> |                                  | spng         | 297.4 ms    | 20.15 MiB/s       | 2.20× slower      |
>
> — [github.com/etemesi254/zune-image/blob/2f9086b07cdf5a36621ce18f79d31…](https://github.com/etemesi254/zune-image/blob/2f9086b07cdf5a36621ce18f79d310f99b751de9/benchmarks/benchmark-results.md)

> | Benchmark | Library | Darwin Time | Darwin Throughput | Darwin Comparison |
>
> — [github.com/etemesi254/zune-image/blob/2f9086b07cdf5a36621ce18f79d31…](https://github.com/etemesi254/zune-image/blob/2f9086b07cdf5a36621ce18f79d310f99b751de9/benchmarks/benchmark-results.md)

> | venice-2000x3000.png | 128.9 | 102.4 | 126% |
>
> — [github.com/google/decoder-benchmarks-for-rust](https://github.com/google/decoder-benchmarks-for-rust)

> Fast image decoders and encoders: `zune-*` image decoders are some of the fastest,
>
> — [github.com/etemesi254/zune-image](https://github.com/etemesi254/zune-image)

</details>
