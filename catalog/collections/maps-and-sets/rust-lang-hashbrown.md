# hashbrown crate (SwissTable with foldhash as default hasher)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Collections](../README.md) › [Maps and sets](README.md)</sub>

**Tier A** (reproducible public benchmark) · 🟡 Holds up in part · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> The SwissTable crate behind std's HashMap; on today's toolchains its speed edge over std is only its foldhash default hasher.

| | |
|---|---|
| **Beats** | std::collections::HashMap, which has used the same hashbrown table since Rust 1.36 but defaults to SipHash-1-3 |
| **Claimed result** | README: around 2x faster than the previous standard library HashMap (no numbers). Against today's std the difference is the hasher: foldhash-f vs SipHash geometric mean 6.21 vs 28.70 ns per operation on Intel Xeon and 4.41 vs 21.94 on Apple M2 (about 4.6x and 5.0x, derived from page data). |
| **Workload** | foldhash suite: hash-only, lookup miss and hit in a 1,000-element map, and set build, over 16 key distributions. |
| **Metric** | time (ns per operation) |
| **Hardware** | Intel Xeon Platinum 8481C; Apple M2 |
| **Toolchain** | Rust 1.80.1, lto = "thin" (foldhash suite) |
| **Measured** | 2024-08 |
| **Adoption** | 3.0k stars |
| **Last commit** | 2026-10-25 |
| **Platforms** | All Rust targets; no_std with alloc; SSE2 or NEON group probing where available |
| **Install** | cargo add hashbrown |
| **Links** | [repository](https://github.com/rust-lang/hashbrown) · [documentation](https://docs.rs/hashbrown) · [benchmark](https://github.com/orlp/foldhash/blob/master/README.md) |

## Verdict

The README's 'around 2x faster than the previous standard library HashMap' compares against std before 1.36; today's std HashMap is hashbrown. The remaining speed difference is the default hasher, foldhash instead of SipHash-1-3, worth about 4.6x to 5x in hashing-heavy benchmarks. std users get the same gain by passing foldhash's hasher, and either way it gives up SipHash's DoS resistance.

**Why tier A:** The remaining gap is the hasher, measured by foldhash's public suite with hardware, Rust version and flags stated; hashbrown's own 2x figure has no numbers in its current README.

## Use it when

- no_std code that needs a hash map.
- You need APIs std does not expose, such as HashTable or allocator parameters.

## Skip it when

- You only want a faster std HashMap: use std's HashMap with foldhash::fast::RandomState and avoid the extra dependency.
- Keys are attacker-controlled: the default foldhash is only minimally DoS-resistant, as the README itself warns.

## How it is fast

It is the same SwissTable as std's HashMap: one control byte per slot and group-wide SIMD scans of those bytes. Since 0.15 it defaults to foldhash, a folded-multiply hasher, where std defaults to SipHash-1-3.

## What it costs

- Weaker HashDoS resistance than std's default.
- MSRV 1.85 for current releases.

## Problems with the evidence

- Benchmarks of hashbrown against std's HashMap on Rust 1.36 or later mostly measure the hasher, not the table.
- The 2x figure refers to std before 1.36 and has no published numbers in the current README.

## Evidence

<details>
<summary>7 quotes from the sources, checked 2026-10-28</summary>

> Since Rust 1.36, the Rust standard library has adopted this implementation for
>
> — [github.com/rust-lang/hashbrown/blob/main/README.md](https://github.com/rust-lang/hashbrown/blob/main/README.md)

> Around 2x faster than the previous standard library `HashMap`.
>
> — [github.com/rust-lang/hashbrown/blob/main/README.md](https://github.com/rust-lang/hashbrown/blob/main/README.md)

> Uses [foldhash](https://github.com/orlp/foldhash) as the default hasher, which is much faster than SipHash.
>
> — [github.com/rust-lang/hashbrown/blob/main/README.md](https://github.com/rust-lang/hashbrown/blob/main/README.md)

> as SipHash, so if that is important to you, you might want to consider using a different hasher.
>
> — [github.com/rust-lang/hashbrown/blob/main/README.md](https://github.com/rust-lang/hashbrown/blob/main/README.md)

> │        geometric_mean       ┆       6.21 ┆       7.01 ┆    7.56 ┆    8.74 ┆   28.70 │
>
> — [github.com/orlp/foldhash/blob/master/README.md](https://github.com/orlp/foldhash/blob/master/README.md)

> │        geometric_mean       ┆       4.41 ┆       4.86 ┆    5.39 ┆    5.71 ┆   21.94 │
>
> — [github.com/orlp/foldhash/blob/master/README.md](https://github.com/orlp/foldhash/blob/master/README.md)

> one with a 2023 Apple M2 CPU, one with a 2023 Intel Xeon Platinum 8481C server CPU, both with stable Rust 1.80.1.
>
> — [github.com/orlp/foldhash/blob/master/README.md](https://github.com/orlp/foldhash/blob/master/README.md)

</details>
