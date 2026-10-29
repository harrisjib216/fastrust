# speedy (binary serialization)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Data formats](../README.md) › [Binary serialization](README.md)</sub>

**Tier A** (reproducible public benchmark) · 🟡 Holds up in part · Open source · `Rust` · `MIT OR Apache-2.0` · 🟡 stale

> Fast custom-derive binary serializer, 3x-34x faster than bincode 1.3 to serialize, but unmaintained and dropped from the main benchmark.

| | |
|---|---|
| **Beats** | bincode 1.3.3 through serde |
| **Claimed result** | Serialize 153.12 µs vs 464.75 µs (log), 141.52 µs vs 4.8569 ms (mesh), 165.88 µs vs 518.86 µs (minecraft_savedata); deserialize 1.3994 ms vs 1.6158 ms, 141.74 µs vs 3.6395 ms, 1.3890 ms vs 1.5229 ms; 3.0x-34x serialize, 1.10x-25.7x deserialize (ratios derived from page data) |
| **Workload** | rust_serialization_benchmark log, mesh and minecraft_savedata datasets with speedy's own Readable/Writable derives (little-endian context). |
| **Metric** | time |
| **Hardware** | AMD EPYC 9V74 (GitHub Actions runner) |
| **OS** | Linux (x86_64-unknown-linux-gnu) |
| **Toolchain** | rustc 1.99.0-nightly (2026-08-06), bench profile lto = true, no target-cpu flag |
| **Measured** | 2026-08-07 |
| **Adoption** | 409 stars |
| **Last commit** | 2024-10-16 |
| **Platforms** | portable |
| **Install** | cargo add speedy |
| **Links** | [repository](https://github.com/koute/speedy) · [documentation](https://docs.rs/speedy) · [benchmark](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md) |

## Verdict

The speed holds: in the 2026-08-07 run speedy 0.8.7 serialized 3.0x-34x and deserialized 1.10x-25.7x faster than bincode 1.3.3. But it has had no commit since 2024-10 or release since 2024-01, the benchmark removed it as unmaintained later that month, and wincode now posts similar numbers while staying bincode-compatible.

**Why tier A:** Third-party benchmark with public code; rustc nightly, the CPU and lto = true are stated.

## Use it when

- Existing code already on speedy that needs no new features

## Skip it when

- New projects: pick wincode (bincode-compatible) or bitcode, which are maintained and post similar or better numbers
- You need fixes or new type support; the repository has been quiet since 2024-10

## How it is fast

Its own Readable/Writable derives read and write fields directly against a fixed endianness context, with no serde Serializer or Visitor layer in between.

## What it costs

- Unmaintained: no release since 0.8.7 (2024-01-14)
- Speedy's own format; not compatible with bincode or serde formats

## Problems with the evidence

- Last measured 2026-08-07; rust_serialization_benchmark deleted its speedy harness in the 2026-08 cleanup under its unmaintained-crate policy
- Shared cloud runner, single run

## Evidence

<details>
<summary>10 quotes from the sources, checked 2026-10-28</summary>

> The goal of this crate is to provide fast, simple and easy binary serialization.
>
> — [github.com/koute/speedy](https://github.com/koute/speedy)

> | [bincode 1.3.3][bincode1] | 464.75 µs | 1.6158 ms | 495.21 µs | 1045784 | 373127 | 311553 | 3.6551 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> | [speedy 0.8.7][speedy] | 153.12 µs | 1.3994 ms | 300.55 µs | 885780 | 362204 | 286248 | 2.8370 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> | [bincode 1.3.3][bincode1] | 4.8569 ms | 3.6395 ms | 6000008 | 5378500 | 5346908 | 7.2277 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> | [speedy 0.8.7][speedy] | 141.52 µs | 141.74 µs | 6000004 | 5378496 | 5346866 | 7.3912 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> | [bincode 1.3.3][bincode1] | 518.86 µs | 1.5229 ms | 721.15 µs | 569975 | 240525 | 231884 | 1.8242 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> | [speedy 0.8.7][speedy] | 165.88 µs | 1.3890 ms | 451.93 µs | 449595 | 234970 | 210192 | 1.4785 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> rustc 1.99.0-nightly (84b36a78a 2026-08-06)
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> Unmaintained crates are subject to removal from the benchmarks.
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/master/CONTR…](https://github.com/djkoloski/rust_serialization_benchmark/blob/master/CONTRIBUTING.md)

> remove unmaintained library bench modules
>
> — [api.github.com/repos/djkoloski/rust_serialization_benchmark/commits…](https://api.github.com/repos/djkoloski/rust_serialization_benchmark/commits/18d9224041)

</details>
