# std HashMap and HashSet (hashbrown SwissTable, Rust 1.36)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Collections](../README.md) › [Maps and sets](README.md)</sub>

**Tier C** (first-party numbers) · ✅ Holds up · Ships with Rust · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Since Rust 1.36, std's HashMap is the hashbrown SwissTable, about 2x faster on lookups than the old Robin Hood table with the same hasher.

| | |
|---|---|
| **Beats** | std::collections::HashMap before Rust 1.36 (Robin Hood open addressing) |
| **Claimed result** | Same FxHash hasher on both sides: find_existing 5,951 vs 2,935 ns/iter (x 2.03), find_nonexisting x 2.03, get_remove_insert 29 vs 25 (x 1.16), grow_by_insertion 160 vs 177 (x 0.90, slower). Against SipHash std HashMap (hasher also changed): find_existing x 8.12. |
| **Workload** | hashbrown's micro-benchmarks: lookups of present and absent keys, get/remove/insert churn, growth by insertion, queue-like use, and create/drop. |
| **Metric** | time (ns/iter) |
| **Measured** | 2019 |
| **Platforms** | All targets with std; SIMD group probing on SSE2 targets, portable group probing elsewhere |
| **Install** | built into std since 1.36 |
| **Links** | [documentation](https://doc.rust-lang.org/std/collections/struct.HashMap.html) · [benchmark](https://github.com/rust-lang/hashbrown/blob/v0.3.0/README.md) |

## Verdict

With the hasher held constant (FxHash on both sides), the SwissTable implementation was about 2x faster on lookups and 1.2x on churn, though slightly slower on growth by insertion. Rust 1.36 adopted it as std's HashMap, so every Rust program already has it; crates that benchmark against pre-2019 std HashMap overstate their gain.

**Why tier C:** Numbers come from the hashbrown author's README at the time of the switch, with no hardware or toolchain stated; the Rust blog confirms the change without numbers.

## Use it when

- Always, by default: it is std's HashMap. Change the hasher (S parameter) for speed rather than switching tables.

## Skip it when

- You need insertion-order iteration (indexmap) or sorted order (BTreeMap).
- You are no_std: use the hashbrown crate directly.

## How it is fast

SwissTable keeps one control byte per slot holding 7 bits of the hash and scans a group of 16 control bytes at a time (with SSE2 where available) to find candidate slots before touching keys. This cuts cache misses and branches on lookups, and the control array costs 1 byte per entry instead of the old table's 8-byte stored hash.

## What it costs

- Growth by insertion was slightly slower than the old table (x 0.90) in the same benchmark.
- Iteration order is arbitrary and differs between runs because of the random seed.

## Problems with the evidence

- Old first-party micro-benchmarks from 2019 with no hardware or compiler stated.
- Most of the gap against std's SipHash HashMap in hashbrown's README (x 8.12) came from the FxHash hasher, not the table.
- std still defaults to SipHash-1-3; the hashbrown crate defaults to foldhash, so hashbrown-vs-std benchmarks mostly measure the hasher.

## Evidence

<details>
<summary>7 quotes from the sources, checked 2026-10-28</summary>

> find_existing      5,951           2,935                    -3,016   -50.68%   x 2.03
>
> — [github.com/rust-lang/hashbrown/blob/v0.3.0/README.md](https://github.com/rust-lang/hashbrown/blob/v0.3.0/README.md)

> find_nonexisting   4,637           2,283                    -2,354   -50.77%   x 2.03
>
> — [github.com/rust-lang/hashbrown/blob/v0.3.0/README.md](https://github.com/rust-lang/hashbrown/blob/v0.3.0/README.md)

> get_remove_insert  29              25                           -4   -13.79%   x 1.16
>
> — [github.com/rust-lang/hashbrown/blob/v0.3.0/README.md](https://github.com/rust-lang/hashbrown/blob/v0.3.0/README.md)

> grow_by_insertion  160             177                          17    10.62%   x 0.90
>
> — [github.com/rust-lang/hashbrown/blob/v0.3.0/README.md](https://github.com/rust-lang/hashbrown/blob/v0.3.0/README.md)

> find_existing      23,831           2,935                   -20,896   -87.68%   x 8.12
>
> — [github.com/rust-lang/hashbrown/blob/v0.3.0/README.md](https://github.com/rust-lang/hashbrown/blob/v0.3.0/README.md)

> faster on average and has lower memory overhead.
>
> — [blog.rust-lang.org/2019/07/04/Rust-1.36.0/](https://blog.rust-lang.org/2019/07/04/Rust-1.36.0/)

> Lower memory usage: only 1 byte of overhead per entry instead of 8.
>
> — [github.com/rust-lang/hashbrown/blob/main/README.md](https://github.com/rust-lang/hashbrown/blob/main/README.md)

</details>
