# faster-hex

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Data formats](../README.md) › [Base64, hex and encoding](README.md)</sub>

**Tier A** (reproducible public benchmark) · ✅ Holds up · Open source · `Rust` · `MIT` · 🟢 active

> SIMD hex encoding and decoding, measured 6x-71x faster than the hex crate by a third party.

| | |
|---|---|
| **Beats** | hex 0.4.3 (hex::encode / hex::decode) |
| **Claimed result** | encode 17.42 vs 102.11 ns (32 B), 42,883.20 vs 3,048,496.30 ns (1 MiB); decode 17.25 vs 107.13 ns (32 B), 118,576.16 vs 6,409,293.90 ns (1 MiB); 5.9x-71x encode, 6.2x-54x decode (ratios derived from page data) |
| **Workload** | libtest #[bench] encode and decode (allocating String/Vec outputs) of 32 B, 256 B, 2 KiB, 16 KiB, 128 KiB and 1 MiB inputs. |
| **Metric** | time |
| **Hardware** | AMD Ryzen 9 7950X |
| **OS** | Linux (x86_64-unknown-linux-gnu) |
| **Toolchain** | rustc 1.83.0-nightly (9ff5fc4ff 2024-10-03), cargo bench |
| **Measured** | 2024-10-06 |
| **Adoption** | 103 stars |
| **Last commit** | 2026-09-23 |
| **Platforms** | x86 SSE4.1/AVX2 (AVX-512BW for validation and decoding), aarch64 NEON, portable fallback; no_std |
| **Install** | cargo add faster-hex |
| **Links** | [repository](https://github.com/nervosnetwork/faster-hex) · [documentation](https://docs.rs/faster-hex) · [benchmark](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md) |

## Verdict

In const-hex's published run (Ryzen 9 7950X, 2024-10), faster-hex 0.10.0 encoded 5.9x-71x and decoded 6.2x-54x faster than hex 0.4 from 32 bytes to 1 MiB. faster-hex 1.0 (2026-09) is a rewrite with no published numbers in text yet.

**Why tier A:** Benchmark run by a third party (const-hex) with public code, the CPU and the rustc nightly stated.

## Use it when

- Bulk hex encoding/decoding on x86_64 or aarch64
- Hash-sized inputs (32 bytes) where it still beat hex by about 6x

## Skip it when

- Versions before 0.10.1 with hex_decode_unchecked on x86: RUSTSEC-2026-0306
- Toolchains older than 1.95, which faster-hex 1.0 requires

## How it is fast

Converts and validates a full SIMD register of bytes per step: SSE4.1 or AVX2 on x86 and NEON on aarch64, with AVX-512BW used for validation and decoding when the CPU has it. A portable fallback covers other targets.

## What it costs

- SIMD paths use unsafe; RUSTSEC-2026-0306 (2026-09-20) found the AVX2 path of the safe hex_decode_unchecked reading past its input in 0.3.0-0.10.0
- MSRV 1.95 for 1.0

## Problems with the evidence

- Numbers are for faster-hex 0.10.0; 1.0 was rewritten and is unmeasured in published text
- const-hex's README says faster-hex is only optimized for x86, which is out of date: faster-hex's README now lists NEON
- Nightly toolchain, libtest bench harness, one machine

## Evidence

<details>
<summary>11 quotes from the sources, checked 2026-10-28</summary>

> The following benchmarks were ran on an AMD Ryzen 9 7950X, compiled with `1.83.0-nightly (9ff5fc4ff 2024-10-03)` on `x86_64-unknown-linux-gnu`.
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

> test encode::faster_hex::bench1_32b ... bench: 17.42 ns/iter (+/- 0.28)
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

> test encode::faster_hex::bench6_1m ... bench: 42,883.20 ns/iter (+/- 1,099.36)
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

> test encode::hex::bench1_32b ... bench: 102.11 ns/iter (+/- 1.75)
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

> test encode::hex::bench6_1m ... bench: 3,048,496.30 ns/iter (+/- 223,992.59)
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

> test decode::faster_hex::bench1_32b ... bench: 17.25 ns/iter (+/- 0.14)
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

> test decode::faster_hex::bench6_1m ... bench: 118,576.16 ns/iter (+/- 2,564.34)
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

> test decode::hex::bench1_32b ... bench: 107.13 ns/iter (+/- 7.28)
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

> test decode::hex::bench6_1m ... bench: 6,409,293.90 ns/iter (+/- 102,747.28)
>
> — [github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158…](https://github.com/DaniPopes/const-hex/blob/7d78addb8fb7a7abbf88010b74e7158487847015/README.md)

> Uses SSE4.1 or AVX2 on x86 and NEON on AArch64 when available, with AVX-512BW acceleration for validation and decoding.
>
> — [github.com/nervosnetwork/faster-hex](https://github.com/nervosnetwork/faster-hex)

> `hex_decode_unchecked` AVX2 path reads past `src`
>
> — [github.com/rustsec/advisory-db/blob/main/crates/faster-hex/RUSTSEC-…](https://github.com/rustsec/advisory-db/blob/main/crates/faster-hex/RUSTSEC-2026-0306.md)

</details>
