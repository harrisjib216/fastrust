# bzip2 0.6 (libbz2-rs-sys Rust backend)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Compression](../README.md) › [Zstandard, LZ4, Snappy and Brotli](README.md)</sub>

**Tier C** (first-party numbers) · ✅ Holds up · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> bzip2 crate 0.6 swapped C libbzip2 for a Rust port: 10-15% fewer cycles to compress, 4-10% fewer to decompress.

| | |
|---|---|
| **Beats** | The C libbzip2 that the bzip2 crate linked through bzip2-sys before 0.6.0 |
| **Claimed result** | Compression: -14.87% CPU cycles (sample3.ref), -14.30% (silesia-small.tar level 1), - 9.66% (level 9); decompression: - 4.48% to -10.00% CPU cycles across six files. |
| **Workload** | Compress sample3.ref and silesia-small.tar at levels 1 and 9; decompress six .bz2 files from 2.5M to 2.3G cycles of work. |
| **Metric** | CPU cycles |
| **Measured** | 2025-06-17 |
| **Adoption** | 174 stars |
| **Last commit** | 2026-01-13 |
| **Platforms** | Any Rust target, including wasm32 and Windows without a C toolchain |
| **Install** | cargo add bzip2 (0.6.0 or later uses the Rust backend by default) |
| **Links** | [repository](https://github.com/trifectatechfoundation/bzip2-rs) · [documentation](https://docs.rs/bzip2) · [benchmark](https://trifectatech.org/blog/bzip2-crate-switches-from-c-to-rust/) |

## Verdict

The Rust port needs 9.66% to 14.87% fewer CPU cycles to compress and 4.48% to 10.00% fewer to decompress across the published inputs. Since bzip2 0.6.0 it is the crate's default, so upgrading gets the gain.

**Why tier C:** First-party CPU-cycle counts with no machine, compiler or flags stated.

## Use it when

- You already depend on bzip2 (often indirectly through zip or tar crates): upgrading to 0.6 gets the speedup
- You need bzip2 on wasm or cross-compiled targets where building C is a problem

## Skip it when

- You need byte-identical behavior with a system libbzip2 shared library for some external reason

## How it is fast

libbz2-rs-sys started as a c2rust translation of libbzip2 and was then cleaned up and tuned with the lessons from zlib-rs. The post does not break down which changes produced the gain.

## What it costs

- Contains unsafe code to mirror the C interface; tested under Miri and audited by Radically Open Security
- libbz2-rs-sys is distributed under the bzip2-1.0.6 license, while the bzip2 crate is MIT OR Apache-2.0

## Problems with the evidence

- Cycle counts, not wall time; the post does not name the machine and notes noisier decompression results on its macOS benchmark machine
- First-party numbers with no CPU named

## Evidence

<details>
<summary>10 quotes from the sources, checked 2026-10-28</summary>

> which uses our rust implementation of the bzip2 algorithm
>
> — [trifectatech.org/blog/bzip2-crate-switches-from-c-to-rust/](https://trifectatech.org/blog/bzip2-crate-switches-from-c-to-rust/)

> Our rust implementation generally outperforms the C implementation, though there are a couple of cases where we only match C performance.
>
> — [trifectatech.org/blog/bzip2-crate-switches-from-c-to-rust/](https://trifectatech.org/blog/bzip2-crate-switches-from-c-to-rust/)

> sample3.ref (level 1)
>
> — [trifectatech.org/blog/bzip2-crate-switches-from-c-to-rust/](https://trifectatech.org/blog/bzip2-crate-switches-from-c-to-rust/)

> silesia-small.tar (level 1)
>
> — [trifectatech.org/blog/bzip2-crate-switches-from-c-to-rust/](https://trifectatech.org/blog/bzip2-crate-switches-from-c-to-rust/)

> silesia-small.tar (level 9)
>
> — [trifectatech.org/blog/bzip2-crate-switches-from-c-to-rust/](https://trifectatech.org/blog/bzip2-crate-switches-from-c-to-rust/)

> -14.87%
>
> — [trifectatech.org/blog/bzip2-crate-switches-from-c-to-rust/](https://trifectatech.org/blog/bzip2-crate-switches-from-c-to-rust/)

> -14.30%
>
> — [trifectatech.org/blog/bzip2-crate-switches-from-c-to-rust/](https://trifectatech.org/blog/bzip2-crate-switches-from-c-to-rust/)

> - 9.66%
>
> — [trifectatech.org/blog/bzip2-crate-switches-from-c-to-rust/](https://trifectatech.org/blog/bzip2-crate-switches-from-c-to-rust/)

> - 4.48%
>
> — [trifectatech.org/blog/bzip2-crate-switches-from-c-to-rust/](https://trifectatech.org/blog/bzip2-crate-switches-from-c-to-rust/)

> -10.00%
>
> — [trifectatech.org/blog/bzip2-crate-switches-from-c-to-rust/](https://trifectatech.org/blog/bzip2-crate-switches-from-c-to-rust/)

</details>
