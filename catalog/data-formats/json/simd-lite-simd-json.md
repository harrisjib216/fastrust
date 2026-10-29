# simd-json (SIMD JSON parser with a serde-style API)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Data formats](../README.md) › [JSON](README.md)</sub>

**Tier A** (reproducible public benchmark) · ✅ Holds up · Open source · `Rust` · `Apache-2.0 OR MIT` · 🟢 active

> Rust port of simdjson with serde support; in an independent benchmark it deserializes 1.2x-1.5x and serializes about 1.6x faster than serde_json.

| | |
|---|---|
| **Beats** | serde_json 1.0.151 serializing and deserializing typed structs |
| **Claimed result** | Deserialize: 4.8315 ms vs 6.1963 ms (log), 73.007 ms vs 102.52 ms (mesh), 4.7901 ms vs 7.1952 ms (minecraft_savedata), 27.082 ms vs 33.502 ms (mk48); 1.24x-1.50x faster. Serialize: 2.2017 ms vs 3.6304 ms, 53.087 ms vs 86.367 ms, 2.2463 ms vs 3.5385 ms, 11.729 ms vs 19.186 ms; 1.58x-1.65x faster (ratios derived from page data) |
| **Workload** | Four datasets round-tripped as JSON with typed Rust structs: log (HTTP request logs, many small strings), mesh (array of float triangles), minecraft_savedata (structured player saves), mk48 (game updates). Criterion time per iteration. simd-json uses simd-json-derive with reused Buffers; serde_json has float_roundtrip enabled. |
| **Metric** | time |
| **Hardware** | AMD EPYC 7763 (4-vCPU GitHub Actions runner under Hyper-V) |
| **OS** | Linux (x86_64-unknown-linux-gnu) |
| **Toolchain** | rustc 1.101.0-nightly (2026-10-04), LLVM 23.1.3, bench profile lto = true, no target-cpu flag |
| **Measured** | 2026-10-05 |
| **Adoption** | 1.4k stars |
| **Last commit** | 2026-10-27 |
| **Platforms** | x86 (AVX2 or SSE4.2 picked at runtime), aarch64 (NEON), wasm (simd128); slower scalar fallback elsewhere |
| **Install** | cargo add simd-json |
| **Links** | [repository](https://github.com/simd-lite/simd-json) · [documentation](https://docs.rs/simd-json) · [benchmark](https://github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbacabe00140bd6132ee5587d8514f06f/README.md) |

## Verdict

In the independent rust_serialization_benchmark run of 2026-10-05, simd-json 0.17.3 deserialized 1.24x-1.50x faster and serialized 1.58x-1.65x faster than serde_json 1.0.151 on all four datasets, with no target-cpu flag. The win is real but modest, far below the C++ simdjson headline figures.

**Why tier A:** Third-party benchmark with public code; the page states rustc 1.101.0-nightly, the CPU, and the repo sets lto = true.

## Use it when

- You parse large JSON payloads and can hand the parser a mutable byte buffer
- Targets with AVX2/SSE4.2 or NEON, where the SIMD paths run
- You can use simd-json-derive or its borrowed value types instead of plain serde

## Skip it when

- The input is a &str you cannot copy or mutate; simd-json parses from &mut [u8]
- Targets without supported SIMD: the README says the fallback is significantly slower
- You depend on exact serde_json behavior; the README notes incompatibilities when used as a drop-in

## How it is fast

Follows the simdjson design: a SIMD pass finds structural characters and validates UTF-8 a block at a time, and a second stage walks the result to build values. Strings are unescaped in place inside the caller's mutable buffer, which avoids copies. On x86 it chooses AVX2 or SSE4.2 code at runtime.

## What it costs

- The README says it uses a lot of unsafe code
- Requires a mutable input buffer
- The README recommends a non-system allocator (snmalloc, mimalloc or jemalloc) for best performance
- MSRV 1.88

## Problems with the evidence

- The benchmark uses simd-json-derive with a reused Buffers struct, not the serde path; serde-based use was not measured here
- serde_json is built with float_roundtrip, which costs it some float-parsing speed
- Shared 4-vCPU cloud runner; the CPU changes between runs (the 2026-08-07 run used an EPYC 9V74)
- simd-json publishes no numbers of its own against serde_json; its README only says it tracks the C++ implementation

## Evidence

<details>
<summary>13 quotes from the sources, checked 2026-10-28</summary>

> | json:<br> [serde_json 1.0.151][serde_json] | 3.6304 ms | 6.1963 ms | † | 1827461 | 470560 | 360727 | 5.5363 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbaca…](https://github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbacabe00140bd6132ee5587d8514f06f/README.md)

> | json:<br> [simd-json 0.17.3][simd-json] | 2.2017 ms | 4.8315 ms | † | 1827461 | 470560 | 360727 | 5.5555 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbaca…](https://github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbacabe00140bd6132ee5587d8514f06f/README.md)

> | json:<br> [serde_json 1.0.151][serde_json] | 86.367 ms | 102.52 ms | 26192883 | 9566084 | 8584671 | 156.90 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbaca…](https://github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbacabe00140bd6132ee5587d8514f06f/README.md)

> | json:<br> [simd-json 0.17.3][simd-json] | 53.087 ms | 73.007 ms | 26192883 | 9566084 | 8584671 | 156.88 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbaca…](https://github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbacabe00140bd6132ee5587d8514f06f/README.md)

> | json:<br> [serde_json 1.0.151][serde_json] | 3.5385 ms | 7.1952 ms | † | 1623191 | 466527 | 359157 | 6.0226 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbaca…](https://github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbacabe00140bd6132ee5587d8514f06f/README.md)

> | json:<br> [simd-json 0.17.3][simd-json] | 2.2463 ms | 4.7901 ms | † | 1623191 | 466527 | 359157 | 5.7474 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbaca…](https://github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbacabe00140bd6132ee5587d8514f06f/README.md)

> | json:<br> [serde_json 1.0.151][serde_json] | 19.186 ms | 33.502 ms | 9390461 | 2391679 | 1842767 | 35.093 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbaca…](https://github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbacabe00140bd6132ee5587d8514f06f/README.md)

> | json:<br> [simd-json 0.17.3][simd-json] | 11.729 ms | 27.082 ms | 9390461 | 2391679 | 1842767 | 35.254 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbaca…](https://github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbacabe00140bd6132ee5587d8514f06f/README.md)

> This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbaca…](https://github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbacabe00140bd6132ee5587d8514f06f/README.md)

> rustc 1.101.0-nightly (282215592 2026-10-04)
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbaca…](https://github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbacabe00140bd6132ee5587d8514f06f/README.md)

> Model name: AMD EPYC 7763 64-Core Processor
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbaca…](https://github.com/djkoloski/rust_serialization_benchmark/blob/9afa003dbacabe00140bd6132ee5587d8514f06f/README.md)

> When no supported SIMD instructions are found, this library will use a fallback implementation, but this is significantly slower.
>
> — [github.com/simd-lite/simd-json](https://github.com/simd-lite/simd-json)

> `simd-json` uses **a lot** of unsafe code.
>
> — [github.com/simd-lite/simd-json](https://github.com/simd-lite/simd-json)

</details>
