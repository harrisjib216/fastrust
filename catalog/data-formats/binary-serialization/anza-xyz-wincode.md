# wincode (bincode-compatible serializer)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Data formats](../README.md) › [Binary serialization](README.md)</sub>

**Tier A** (reproducible public benchmark) · ✅ Holds up · Open source · `Rust` · `Apache-2.0` · 🟢 active

> Drop-in for bincode 1's wire format that serializes 3x-33x and deserializes 1.1x-26x faster than bincode 1.3.3 through serde.

| | |
|---|---|
| **Beats** | bincode 1.3.3 through serde (same wire format) |
| **Claimed result** | Serialize 140.61 µs vs 464.75 µs (log), 148.62 µs vs 4.8569 ms (mesh), 140.23 µs vs 518.86 µs (minecraft_savedata); deserialize 1.4340 ms vs 1.6158 ms, 140.71 µs vs 3.6395 ms, 1.3652 ms vs 1.5229 ms; 3.3x-32.7x serialize, 1.12x-25.9x deserialize (ratios derived from page data) |
| **Workload** | rust_serialization_benchmark log, mesh and minecraft_savedata datasets; serialize into a pre-sized buffer and deserialize into owned types; wincode derives SchemaWrite/SchemaRead, bincode 1.3.3 uses serde. |
| **Metric** | time |
| **Hardware** | AMD EPYC 9V74 (GitHub Actions runner) |
| **OS** | Linux (x86_64-unknown-linux-gnu) |
| **Toolchain** | rustc 1.99.0-nightly (2026-08-06), bench profile lto = true, no target-cpu flag |
| **Measured** | 2026-08-07 |
| **Adoption** | 237 stars |
| **Last commit** | 2026-10-28 |
| **Platforms** | portable |
| **Install** | cargo add wincode --features derive |
| **Links** | [repository](https://github.com/anza-xyz/wincode) · [documentation](https://docs.rs/wincode) · [benchmark](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md) |

## Verdict

Writing bincode 1.3.3's format (identical output sizes on log and mesh), wincode 0.6.0 serialized 3.3x-32.7x faster and deserialized 1.12x-25.9x faster in the independent 2026-08-07 run, with the large gains on float-heavy mesh data. bincode stopped development in 2025-12 and its maintainers list wincode as an alternative.

**Why tier A:** Third-party benchmark with public code; rustc nightly, the CPU and lto = true are stated.

## Use it when

- Replacing bincode 1.x where existing stored or on-the-wire bytes must stay readable
- Plain-data-heavy payloads such as float arrays, where the gap was 26x-33x
- Leaving the unmaintained bincode crate

## Skip it when

- String-heavy data where deserialization only gained 1.12x-1.13x; consider bitcode if you can change the format
- Types that only implement serde traits and cannot take wincode's derives

## How it is fast

Derive macros generate SchemaWrite/SchemaRead code that writes fields straight into the output buffer and initializes values in place when reading, instead of going through serde's Serializer and Visitor layers. The README describes the design as focused on in-place initialization and direct memory writes.

## What it costs

- MSRV 1.89, edition 2024
- Newer crate (0.x) maintained by Anza; v0.1.2 had three external security reviews per the README

## Problems with the evidence

- Compatibility is per configuration: on minecraft_savedata the benchmark recorded 566975 bytes for wincode vs 569975 for bincode 1.3.3, so check your types
- Single run on a shared runner; bincode was removed from later runs
- speedy, rkyv and wincode all land near 140-150 µs on mesh while bincode 1.3.3 takes about 4.9 ms, so much of the mesh gap reflects bincode-through-serde's per-element cost on float arrays

## Evidence

<details>
<summary>10 quotes from the sources, checked 2026-10-28</summary>

> Fast, bincode‑compatible serializer/deserializer focused on in‑place initialization and direct memory writes.
>
> — [github.com/anza-xyz/wincode](https://github.com/anza-xyz/wincode)

> | [bincode 1.3.3][bincode1] | 464.75 µs | 1.6158 ms | 495.21 µs | 1045784 | 373127 | 311553 | 3.6551 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> | [wincode 0.6.0][wincode] | 140.61 µs | 1.4340 ms | 302.08 µs | 1045784 | 373127 | 311553 | 3.0649 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> | [bincode 1.3.3][bincode1] | 4.8569 ms | 3.6395 ms | 6000008 | 5378500 | 5346908 | 7.2277 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> | [wincode 0.6.0][wincode] | 148.62 µs | 140.71 µs | 6000008 | 5378500 | 5346908 | 7.2479 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> | [bincode 1.3.3][bincode1] | 518.86 µs | 1.5229 ms | 721.15 µs | 569975 | 240525 | 231884 | 1.8242 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> | [wincode 0.6.0][wincode] | 140.23 µs | 1.3652 ms | 456.71 µs | 566975 | 239350 | 231475 | 1.8026 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> rustc 1.99.0-nightly (84b36a78a 2026-08-06)
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> Due to a doxxing and harassment incident, the bincode team has taken the decision to cease development permanently.
>
> — [github.com/rustsec/advisory-db/blob/main/crates/bincode/RUSTSEC-202…](https://github.com/rustsec/advisory-db/blob/main/crates/bincode/RUSTSEC-2025-0141.md)

> * [wincode](https://crates.io/crates/wincode)
>
> — [github.com/rustsec/advisory-db/blob/main/crates/bincode/RUSTSEC-202…](https://github.com/rustsec/advisory-db/blob/main/crates/bincode/RUSTSEC-2025-0141.md)

</details>
