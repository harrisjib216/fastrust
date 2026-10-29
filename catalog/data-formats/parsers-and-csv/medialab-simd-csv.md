# simd-csv (SIMD CSV reader)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Data formats](../README.md) › [Parsers and CSV](README.md)</sub>

**Tier C** (first-party numbers) · ✅ Holds up · Open source · `Rust` · `MIT` · 🟢 active

> CSV reader that pairs a state machine with SIMD byte searching; its author measures up to 4x-6x over the csv crate on long-cell data.

| | |
|---|---|
| **Beats** | the csv crate's Reader (csv 1.x) |
| **Claimed result** | vs csv: articles.csv ~4.8x split / ~4.7x zero_copy / ~4.0x copy; tweets.csv ~6.0x / ~2.9x / ~2.5x; quote-always.csv ~1.1x / ~1.2x / ~1.0x; worst-case.csv ~1.0x / ~1.1x / ~0.9x |
| **Workload** | Streaming CSV files of 100MB to 11GB from SSD, including I/O: long text columns (articles, mediapart), many sparse columns (tweets), numbers, and a worst case of single-character cells. |
| **Metric** | throughput |
| **Hardware** | x86_64 with AVX2 (model not stated) |
| **Measured** | 2025-11 |
| **Adoption** | 21 stars |
| **Last commit** | 2026-09-16 |
| **Platforms** | x86_64 SSE2 with AVX2 detected at runtime, aarch64 NEON, wasm simd128; SWAR or scalar fallback elsewhere |
| **Install** | cargo add simd-csv |
| **Links** | [repository](https://github.com/medialab/simd-csv) · [documentation](https://docs.rs/simd-csv) · [benchmark](https://github.com/medialab/simd-csv/blob/37d1b3983af7aacf8c2e0314ecc1a7b06104b5cc/README.md) |

## Verdict

The author's table, measured including I/O from SSD on AVX2 hardware, reports up to ~6.0x with the low-level Splitter, ~1.1x-~4.7x with ZeroCopyReader and ~0.9x-~4.0x with the copying Reader. The README is upfront that gains depend on cell length and that dense or always-quoted data gets little or nothing; no independent run exists.

**Why tier C:** First-party, approximate ratios; CPU model, toolchain and crate versions are not stated and several datasets are private.

## Use it when

- Large CSV/TSV files with long or text-heavy cells
- Pipelines that can use the Splitter or ZeroCopyReader instead of owned records
- Data quoted only when needed

## Skip it when

- Files written with QUOTE_ALWAYS or with very short cells, where the table shows ~0.9x-~1.2x
- You need the csv crate's serde integration and flexibility; the README says simd-csv is less flexible

## How it is fast

Keeps a conventional CSV state machine but finds the next delimiter, quote or newline by comparing 16 or 32 bytes at once and reading the match mask, so it branches per structural character instead of per byte. Unquoted data uses an amortized memchr-style search; quoted fields use memchr's unrolled routines.

## What it costs

- Less flexible and less user-friendly than csv, per the README
- Has no concept of beginning of field, so a quote opened mid-field can corrupt output where traditional parsers cope, per the README

## Problems with the evidence

- Approximate (~) ratios with no absolute times, CPU model or versions
- Includes I/O, which dilutes parser differences
- Author-run on datasets that are mostly not public; no third-party confirmation
- Small project (21 stars)

## Evidence

<details>
<summary>7 quotes from the sources, checked 2026-10-28</summary>

> | articles.csv       | ~4.8x | ~4.7x     | ~4.0x |
>
> — [github.com/medialab/simd-csv/blob/37d1b3983af7aacf8c2e0314ecc1a7b06…](https://github.com/medialab/simd-csv/blob/37d1b3983af7aacf8c2e0314ecc1a7b06104b5cc/README.md)

> | tweets.csv         | ~6.0x | ~2.9x     | ~2.5x |
>
> — [github.com/medialab/simd-csv/blob/37d1b3983af7aacf8c2e0314ecc1a7b06…](https://github.com/medialab/simd-csv/blob/37d1b3983af7aacf8c2e0314ecc1a7b06104b5cc/README.md)

> | quote-always.csv   | ~1.1x | ~1.2x     | ~1.0x |
>
> — [github.com/medialab/simd-csv/blob/37d1b3983af7aacf8c2e0314ecc1a7b06…](https://github.com/medialab/simd-csv/blob/37d1b3983af7aacf8c2e0314ecc1a7b06104b5cc/README.md)

> | worst-case.csv     | ~1.0x | ~1.1x     | ~0.9x |
>
> — [github.com/medialab/simd-csv/blob/37d1b3983af7aacf8c2e0314ecc1a7b06…](https://github.com/medialab/simd-csv/blob/37d1b3983af7aacf8c2e0314ecc1a7b06104b5cc/README.md)

> Comparison is done including IO, on `x86_64` with `avx2` support.
>
> — [github.com/medialab/simd-csv/blob/37d1b3983af7aacf8c2e0314ecc1a7b06…](https://github.com/medialab/simd-csv/blob/37d1b3983af7aacf8c2e0314ecc1a7b06104b5cc/README.md)

> Sometimes you will go up to ~8 times faster, sometimes you will only go as fast as scalar code.
>
> — [github.com/medialab/simd-csv/blob/37d1b3983af7aacf8c2e0314ecc1a7b06…](https://github.com/medialab/simd-csv/blob/37d1b3983af7aacf8c2e0314ecc1a7b06104b5cc/README.md)

> Is is less flexible and user-friendly than the [`csv`](https://docs.rs/csv/) crate,
>
> — [github.com/medialab/simd-csv/blob/37d1b3983af7aacf8c2e0314ecc1a7b06…](https://github.com/medialab/simd-csv/blob/37d1b3983af7aacf8c2e0314ecc1a7b06104b5cc/README.md)

</details>
