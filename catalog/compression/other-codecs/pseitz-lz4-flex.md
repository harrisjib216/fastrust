# lz4_flex (pure-Rust LZ4)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Compression](../README.md) › [Zstandard, LZ4, Snappy and Brotli](README.md)</sub>

**Tier A** (reproducible public benchmark) · 🟡 Holds up in part · Open source · `Rust` · `MIT` · 🟢 active

> Pure-Rust LZ4 that matches the C library when unsafe features are on; the default safe build is up to 20% slower than C.

| | |
|---|---|
| **Beats** | lz4 1.9.3 C library through the lzzz bindings, block format |
| **Claimed result** | Unsafe build vs lz4 1.9.3: 66Kb JSON compress 1615 vs 1469 MiB/s (1.10x), decompress 5512 vs 5313 MiB/s (1.04x); 10 Mb dickens compress 347 vs 324 MiB/s (1.07x), decompress 2734 vs 2759 MiB/s (0.99x). Default safe build: 1272 / 4540 MiB/s on JSON and 259 / 2338 MiB/s on dickens, slower than C (ratios derived from page data). |
| **Workload** | Block-format compress and decompress of a 66 KB JSON file and the 10 MB dickens text file. |
| **Metric** | throughput |
| **Hardware** | AMD Ryzen 7 5900HX, CPU boost disabled, performance governor |
| **OS** | Manjaro Linux |
| **Toolchain** | rustc 1.69.0 (2023-04-16), cargo bench |
| **Measured** | 2023-06 |
| **Adoption** | 619 stars |
| **Last commit** | 2026-10-25 |
| **Platforms** | Any Rust target including 32-bit; no_std with the block format |
| **Install** | cargo add lz4_flex |
| **Links** | [repository](https://github.com/PSeitz/lz4_flex) · [documentation](https://docs.rs/lz4_flex) · [benchmark](https://github.com/PSeitz/lz4_flex/blob/ca019ecef51ef6b516c415b69a6ee6f427f0d81c/README.md) |

## Verdict

With unsafe features enabled it roughly matches the C library: 10% faster compression on small JSON, 7% faster on dickens, and decompression within 4% either way. The default safe build is 13% to 20% slower than C.

**Why tier A:** Public criterion benchmark in the repository, with CPU, OS and rustc version stated.

## Use it when

- You want LZ4 without a C toolchain (wasm, cross-compiling, no_std)
- You can enable the unsafe features for trusted input and want C-level speed

## Skip it when

- You need LZ4 HC high-compression modes, which lz4_flex does not implement
- You expect it to beat the C library: with defaults it is slower

## How it is fast

A from-scratch port of the LZ4 block and frame formats with a hash-table match finder and wide copies. Disabling the safe-encode and safe-decode features swaps bounds-checked slice access for unchecked pointer reads and writes, which is where it catches up with C.

## What it costs

- The fast configuration turns off bounds-checked decoding, which matters for untrusted input
- No LZ4 HC (high-compression) levels

## Problems with the evidence

- Results are for v0.11.0 (2023) on rustc 1.69; the crate is now at 0.14.0
- Two input files; the full criterion results are published only as SVG charts

## Evidence

<details>
<summary>10 quotes from the sources, checked 2026-10-28</summary>

> AMD Ryzen 7 5900HX, rustc 1.69.0 (84c898d65 2023-04-16), Manjaro, CPU Boost Disabled, CPU Governor: Performance
>
> — [github.com/PSeitz/lz4_flex/blob/ca019ecef51ef6b516c415b69a6ee6f427f…](https://github.com/PSeitz/lz4_flex/blob/ca019ecef51ef6b516c415b69a6ee6f427f0d81c/README.md)

> | lz4_flex unsafe | 1615 MiB/s | 5512 MiB/s | 0.2284 |
>
> — [github.com/PSeitz/lz4_flex/blob/ca019ecef51ef6b516c415b69a6ee6f427f…](https://github.com/PSeitz/lz4_flex/blob/ca019ecef51ef6b516c415b69a6ee6f427f0d81c/README.md)

> | lz4_flex safe | 1272 MiB/s | 4540 MiB/s | 0.2284 |
>
> — [github.com/PSeitz/lz4_flex/blob/ca019ecef51ef6b516c415b69a6ee6f427f…](https://github.com/PSeitz/lz4_flex/blob/ca019ecef51ef6b516c415b69a6ee6f427f0d81c/README.md)

> | lzzz (lz4 1.9.3) | 1469 MiB/s | 5313 MiB/s | 0.2283 |
>
> — [github.com/PSeitz/lz4_flex/blob/ca019ecef51ef6b516c415b69a6ee6f427f…](https://github.com/PSeitz/lz4_flex/blob/ca019ecef51ef6b516c415b69a6ee6f427f0d81c/README.md)

> | lz4_flex unsafe | 347 MiB/s | 2734 MiB/s | 0.6372 |
>
> — [github.com/PSeitz/lz4_flex/blob/ca019ecef51ef6b516c415b69a6ee6f427f…](https://github.com/PSeitz/lz4_flex/blob/ca019ecef51ef6b516c415b69a6ee6f427f0d81c/README.md)

> | lz4_flex safe | 259 MiB/s | 2338 MiB/s | 0.6372 |
>
> — [github.com/PSeitz/lz4_flex/blob/ca019ecef51ef6b516c415b69a6ee6f427f…](https://github.com/PSeitz/lz4_flex/blob/ca019ecef51ef6b516c415b69a6ee6f427f0d81c/README.md)

> | lzzz (lz4 1.9.3) | 324 MiB/s | 2759 MiB/s | 0.6372 |
>
> — [github.com/PSeitz/lz4_flex/blob/ca019ecef51ef6b516c415b69a6ee6f427f…](https://github.com/PSeitz/lz4_flex/blob/ca019ecef51ef6b516c415b69a6ee6f427f0d81c/README.md)

> ### Results v0.11.0 02-06-2023 (safe-decode and safe-encode off)
>
> — [github.com/PSeitz/lz4_flex/blob/ca019ecef51ef6b516c415b69a6ee6f427f…](https://github.com/PSeitz/lz4_flex/blob/ca019ecef51ef6b516c415b69a6ee6f427f0d81c/README.md)

> 66Kb JSON
>
> — [github.com/PSeitz/lz4_flex/blob/ca019ecef51ef6b516c415b69a6ee6f427f…](https://github.com/PSeitz/lz4_flex/blob/ca019ecef51ef6b516c415b69a6ee6f427f0d81c/README.md)

> 10 Mb dickens
>
> — [github.com/PSeitz/lz4_flex/blob/ca019ecef51ef6b516c415b69a6ee6f427f…](https://github.com/PSeitz/lz4_flex/blob/ca019ecef51ef6b516c415b69a6ee6f427f0d81c/README.md)

</details>
