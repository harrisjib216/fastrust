# flexon (SIMD JSON parser)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Data formats](../README.md) › [JSON](README.md)</sub>

**Tier A** (reproducible public benchmark) · ✅ Holds up · Open source · `Rust` · `MIT` · 🟢 active

> Young SIMD JSON parser with serde support; an independent benchmark has it deserializing 1.5x-1.9x faster than serde_json.

| | |
|---|---|
| **Beats** | serde_json 1.0.151 deserializing typed structs |
| **Claimed result** | Deserialize: 3.6965 ms vs 6.1963 ms (log), 53.131 ms vs 102.52 ms (mesh), 4.4470 ms vs 7.1952 ms (minecraft_savedata), 22.583 ms vs 33.502 ms (mk48); 1.48x-1.93x faster. Serialize: 2.3589 ms vs 3.6304 ms, 66.328 ms vs 86.367 ms, 2.3604 ms vs 3.5385 ms, 12.812 ms vs 19.186 ms; 1.30x-1.54x faster (ratios derived from page data) |
| **Workload** | Same four datasets as the rest of rust_serialization_benchmark, round-tripped as JSON through serde derives; flexon deserializes from its NullPadded source buffer. |
| **Metric** | time |
| **Hardware** | AMD EPYC 7763 (4-vCPU GitHub Actions runner under Hyper-V) |
| **OS** | Linux (x86_64-unknown-linux-gnu) |
| **Toolchain** | rustc 1.101.0-nightly (2026-10-04), bench profile lto = true, no target-cpu flag |
| **Measured** | 2026-10-05 |
| **Adoption** | 68 stars |
| **Last commit** | 2026-10-15 |
| **Platforms** | x86_64 (SSE2 always, AVX2 via runtime detection for skipping APIs); SWAR elsewhere; std and alloc are optional features |
| **Install** | cargo add flexon |
| **Links** | [repository](https://github.com/cyruspyre/flexon) · [documentation](https://docs.rs/flexon) · [benchmark](https://github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbacabe00140bd6132ee5587d8514f06f/README.md) |

## Verdict

In the independent rust_serialization_benchmark run of 2026-10-05, flexon 0.4.9 deserialized 1.48x-1.93x faster than serde_json 1.0.151 on all four datasets and beat simd-json's deserialization on each. Serialization was 1.30x-1.54x faster than serde_json but behind simd-json.

**Why tier A:** Third-party benchmark with public code; rustc 1.101.0-nightly, the CPU and LTO are stated.

## Use it when

- Deserializing JSON into serde types where decode speed matters
- Config-style JSON that needs comments, trailing commas or spans, which flexon supports as opt-in features
- Extracting one value from a large document with get_from

## Skip it when

- Serialization-heavy workloads: simd-json serialized faster on all four datasets
- You need a long-established crate: first release 2025-10, 68 stars, 0.x API

## How it is fast

Uses hardware SIMD where available (SSE2 is always on for x86_64) and SWAR elsewhere to scan JSON. Runtime detection enables AVX2 for its unchecked field-skipping APIs. The benchmark parses from flexon's NullPadded source type, a padded input buffer.

## What it costs

- Pre-1.0 crate from a single maintainer
- Input goes through a padded source type in the fastest path
- Comment and span support are compile-time options because they slow general parsing, per the README

## Problems with the evidence

- Single source of numbers; the README's Performance section is empty
- Shared 4-vCPU cloud runner, one run
- Only typed serde round trips were measured; untyped Value parsing was not

## Evidence

<details>
<summary>11 quotes from the sources, checked 2026-10-28</summary>

> | json:<br> [flexon 0.4.9][flexon] | 2.3589 ms | 3.6965 ms | † | 1827461 | 470560 | 360727 | 5.9821 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbaca…](https://github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbacabe00140bd6132ee5587d8514f06f/README.md)

> | json:<br> [serde_json 1.0.151][serde_json] | 3.6304 ms | 6.1963 ms | † | 1827461 | 470560 | 360727 | 5.5363 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbaca…](https://github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbacabe00140bd6132ee5587d8514f06f/README.md)

> | json:<br> [flexon 0.4.9][flexon] | 66.328 ms | 53.131 ms | 26192883 | 9566084 | 8584671 | 157.34 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbaca…](https://github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbacabe00140bd6132ee5587d8514f06f/README.md)

> | json:<br> [serde_json 1.0.151][serde_json] | 86.367 ms | 102.52 ms | 26192883 | 9566084 | 8584671 | 156.90 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbaca…](https://github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbacabe00140bd6132ee5587d8514f06f/README.md)

> | json:<br> [flexon 0.4.9][flexon] | 2.3604 ms | 4.4470 ms | † | 1623191 | 466527 | 359157 | 5.8095 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbaca…](https://github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbacabe00140bd6132ee5587d8514f06f/README.md)

> | json:<br> [serde_json 1.0.151][serde_json] | 3.5385 ms | 7.1952 ms | † | 1623191 | 466527 | 359157 | 6.0226 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbaca…](https://github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbacabe00140bd6132ee5587d8514f06f/README.md)

> | json:<br> [flexon 0.4.9][flexon] | 12.812 ms | 22.583 ms | 9390461 | 2391679 | 1842767 | 35.161 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbaca…](https://github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbacabe00140bd6132ee5587d8514f06f/README.md)

> | json:<br> [serde_json 1.0.151][serde_json] | 19.186 ms | 33.502 ms | 9390461 | 2391679 | 1842767 | 35.093 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbaca…](https://github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbacabe00140bd6132ee5587d8514f06f/README.md)

> This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbaca…](https://github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbacabe00140bd6132ee5587d8514f06f/README.md)

> rustc 1.101.0-nightly (282215592 2026-10-04)
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbaca…](https://github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbacabe00140bd6132ee5587d8514f06f/README.md)

> On `x86_64`, SSE2 is used regardless of this flag as it is a baseline feature.
>
> — [github.com/cyruspyre/flexon](https://github.com/cyruspyre/flexon)

</details>
