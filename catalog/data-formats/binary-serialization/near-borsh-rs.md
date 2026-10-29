# borsh (Binary Object Representation Serializer for Hashing)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Data formats](../README.md) › [Binary serialization](README.md)</sub>

**Tier A** (reproducible public benchmark) · ❌ Doesn't hold up · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Canonical binary format from NEAR for hashing and consensus; benchmarks at the same speed as bincode 1.3, not faster.

| | |
|---|---|
| **Beats** | bincode 1.3.3 through serde |
| **Claimed result** | Serialize 481.77 µs vs 464.75 µs (log, slower), 5.3728 ms vs 4.8569 ms (mesh, slower), 485.09 µs vs 518.86 µs (minecraft_savedata, 1.07x faster); deserialize 1.6997 ms vs 1.6158 ms (slower), 3.7284 ms vs 3.6395 ms (slower), 1.4610 ms vs 1.5229 ms (1.04x faster) (ratios derived from page data) |
| **Workload** | rust_serialization_benchmark log, mesh and minecraft_savedata datasets with borsh's own derives. |
| **Metric** | time |
| **Hardware** | AMD EPYC 9V74 (GitHub Actions runner) |
| **OS** | Linux (x86_64-unknown-linux-gnu) |
| **Toolchain** | rustc 1.99.0-nightly (2026-08-06), bench profile lto = true, no target-cpu flag |
| **Measured** | 2026-08-07 |
| **Adoption** | 434 stars |
| **Last commit** | 2026-10-23 |
| **Platforms** | portable; no_std |
| **Install** | cargo add borsh --features derive |
| **Links** | [repository](https://github.com/near/borsh-rs) · [documentation](https://docs.rs/borsh) · [benchmark](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md) |

## Verdict

borsh.io says skipping serde makes borsh faster than bincode in some cases. In the independent 2026-08-07 run borsh 1.8.0 was within 11% of bincode 1.3.3 everywhere: slower on log and mesh, and 4%-7% faster only on minecraft_savedata. That is not a measurable speed advantage; borsh's real selling point is a canonical, hash-stable encoding.

**Why tier A:** Third-party benchmark with public code; rustc nightly, the CPU and lto = true are stated.

## Use it when

- You need a bijective encoding whose bytes are stable for hashing or signing
- Interop with NEAR or Solana programs that already speak borsh

## Skip it when

- Speed is the reason for switching from bincode: wincode, bitcode or rkyv are several times faster in the same run

## How it is fast

Uses its own BorshSerialize/BorshDeserialize derives instead of serde and a simple fixed layout (little-endian integers, length-prefixed collections). The benchmark shows this lands at about bincode's speed.

## What it costs

- RUSTSEC-2023-0033 (unsound parsing of non-Copy zero-sized types) affected versions before 1.0.0-alpha.1 and 0.10.4

## Problems with the evidence

- borsh.io's own benchmarks are charts from a Google Cloud n1-standard-2 run with no text numbers
- Shared cloud runner, single run

## Evidence

<details>
<summary>8 quotes from the sources, checked 2026-10-28</summary>

> In Rust, Borsh achieves high performance by opting out from
>
> — [borsh.io/](https://borsh.io/)

> | [bincode 1.3.3][bincode1] | 464.75 µs | 1.6158 ms | 495.21 µs | 1045784 | 373127 | 311553 | 3.6551 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> | [borsh 1.8.0][borsh] | 481.77 µs | 1.6997 ms | † | 885780 | 362204 | 286248 | 3.1120 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> | [bincode 1.3.3][bincode1] | 4.8569 ms | 3.6395 ms | 6000008 | 5378500 | 5346908 | 7.2277 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> | [borsh 1.8.0][borsh] | 5.3728 ms | 3.7284 ms | 6000004 | 5378496 | 5346866 | 7.2775 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> | [bincode 1.3.3][bincode1] | 518.86 µs | 1.5229 ms | 721.15 µs | 569975 | 240525 | 231884 | 1.8242 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> | [borsh 1.8.0][borsh] | 485.09 µs | 1.4610 ms | † | 446595 | 234236 | 209834 | 1.4723 ms |
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

> rustc 1.99.0-nightly (84b36a78a 2026-08-06)
>
> — [github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564f…](https://github.com/djkoloski/rust_serialization_benchmark/blob/60c1cd50564fb6e62cc4928e53933d258de724d8/README.md)

</details>
