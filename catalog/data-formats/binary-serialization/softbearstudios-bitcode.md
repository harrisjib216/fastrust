# bitcode (compact binary Encode/Decode)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Data formats](../README.md) › [Binary serialization](README.md)</sub>

**Tier A** (reproducible public benchmark) · ✅ Holds up · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Binary format with its own derive that serializes 4x-6x and deserializes 1.4x-2.9x faster than bincode 1.3 via serde, with smaller output.

| | |
|---|---|
| **Beats** | bincode 1.3.3 through serde; also serde_json 1.0.151 |
| **Claimed result** | vs bincode 1.3.3: serialize 95.792 µs vs 464.75 µs (log), 81.495 µs vs 518.86 µs (minecraft_savedata), 1.2197 ms vs 4.8569 ms (mesh); deserialize 1.1430 ms vs 1.6158 ms, 1.0356 ms vs 1.5229 ms, 1.2589 ms vs 3.6395 ms; size 703710 vs 1045784 bytes (log). vs serde_json (2026-10-05 run): serialize 146.23 µs vs 3.6304 ms, deserialize 1.4627 ms vs 6.1963 ms (log). Ratios 3.98x-6.37x serialize and 1.41x-2.89x deserialize vs bincode (derived from page data) |
| **Workload** | rust_serialization_benchmark datasets log (many small strings), minecraft_savedata (structured saves) and mesh (float triangles); serialize into a buffer and deserialize into owned Rust types; bitcode uses its own derive, bincode 1.3.3 goes through serde. |
| **Metric** | time |
| **Hardware** | AMD EPYC 9V74 GitHub Actions runner (2026-08-07 run); AMD EPYC 7763 runner (2026-10-05 run) |
| **OS** | Linux (x86_64-unknown-linux-gnu) |
| **Toolchain** | rustc 1.99.0-nightly (2026-08-06) and 1.101.0-nightly (2026-10-04); bench profile lto = true; no target-cpu flag |
| **Measured** | 2026-08-07 |
| **Adoption** | 676 stars |
| **Last commit** | 2026-06-28 |
| **Platforms** | portable; no_std with alloc |
| **Install** | cargo add bitcode |
| **Links** | [repository](https://github.com/SoftbearStudios/bitcode) · [documentation](https://docs.rs/bitcode) · [benchmark](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md) |

## Verdict

In the independent 2026-08-07 run, bitcode 0.6.9 serialized 3.98x-6.37x faster and deserialized 1.41x-2.89x faster than bincode 1.3.3 via serde, with the smallest output on log and minecraft_savedata. On float-heavy mesh data it was slower than bincode 2 and about 9x slower to serialize than rkyv, speedy or wincode.

**Why tier A:** Third-party benchmark with public code; rustc nightly, the CPU and lto = true are stated.

## Use it when

- Rust-to-Rust messages or caches where both serialize speed and size matter
- String- and struct-heavy data like the log and minecraft_savedata sets
- Data that will be compressed afterwards; the grouped layout compresses well

## Skip it when

- Large arrays of plain floats: rkyv, speedy and wincode serialized mesh about 9x faster
- You need a format that stays stable across major versions; the README lists that as a non-goal
- Other languages must read the data

## How it is fast

Groups all instances of each field together (column-wise, after tree-buf) and stores integers in as few bits as the values need. Code is written so LLVM can auto-vectorize it, and validation runs up front on typed vectors before deserialization.

## What it costs

- Format may change between major versions
- Not self-describing; schema lives in the Rust types
- Rust-only

## Problems with the evidence

- bincode was removed from the benchmark in 2026-08 as unmaintained, so the bincode comparison comes from the 2026-08-07 run and the serde_json comparison from the 2026-10-05 run, on different runner CPUs
- Shared cloud runners; single runs
- bitcode's derive is compared with bincode 1.3.3 through serde, so part of the gain is avoiding serde

## Evidence

<details>
<summary>14 quotes from the sources, checked 2026-10-28</summary>

> All instances of each field are grouped together making compression easier
>
> — [github.com/SoftbearStudios/bitcode](https://github.com/SoftbearStudios/bitcode)

> Stable format across major versions
>
> — [github.com/SoftbearStudios/bitcode](https://github.com/SoftbearStudios/bitcode)

> | [bincode 1.3.3][bincode1] | 464.75 µs | 1.6158 ms | 495.21 µs | 1045784 | 373127 | 311553 | 3.6551 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> | [bitcode 0.6.9][bitcode] | 95.792 µs | 1.1430 ms | 48.788 µs | 703710 | 288826 | 227322 | 1.9864 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> | [bincode 1.3.3][bincode1] | 518.86 µs | 1.5229 ms | 721.15 µs | 569975 | 240525 | 231884 | 1.8242 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> | [bitcode 0.6.9][bitcode] | 81.495 µs | 1.0356 ms | 117.10 µs | 327688 | 200947 | 182040 | 625.33 µs |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> | [bincode 1.3.3][bincode1] | 4.8569 ms | 3.6395 ms | 6000008 | 5378500 | 5346908 | 7.2277 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> | [bitcode 0.6.9][bitcode] | 1.2197 ms | 1.2589 ms | 6000006 | 5182295 | 4921841 | 11.159 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> | [bincode 2.0.1][bincode] | 897.61 µs | 859.41 µs | 6000005 | 5378497 | 5346882 | 7.3744 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> | [rkyv 0.8.18][rkyv] | 140.04 µs |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> rustc 1.99.0-nightly (84b36a78a 2026-08-06)
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> Model name: AMD EPYC 9V74 80-Core Processor
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> | [bitcode 0.6.9][bitcode] | 146.23 µs | 1.4627 ms | 62.383 µs | 703710 | 288826 | 227322 | 2.4072 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbaca…](https://github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbacabe00140bd6132ee5587d8514f06f/README.md)

> | json:<br> [serde_json 1.0.151][serde_json] | 3.6304 ms | 6.1963 ms | † | 1827461 | 470560 | 360727 | 5.5363 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbaca…](https://github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbacabe00140bd6132ee5587d8514f06f/README.md)

</details>
