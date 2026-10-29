# rustc-hash (FxHasher, rewritten in 2.0)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Collections](../README.md) › [Hashers for maps and sets](README.md)</sub>

**Tier B** (independent measurement) · 🟡 Holds up in part · Open source · `Rust` · `Apache-2.0 OR MIT` · 🟢 active

> The rustc compiler's unkeyed FxHasher; fastest on integer keys, with no DoS resistance and known collision patterns.

| | |
|---|---|
| **Beats** | std HashMap's RandomState (SipHash-1-3) |
| **Claimed result** | fxhash 0.2.1 (the pre-2.0 algorithm) on AMD EPYC: u64 lookup hit 2.03 vs 14.74 ns for SipHash (about 7.3x, derived from page data); keys varying only in the top bits: set build 82.23 vs 11.50 ns (about 7x slower than SipHash, derived from page data); 1 KB keys hash-only 156.68 vs 47.21 ns for foldhash. rustc-hash 2 inside rustc: instruction count -0.1% mean over 18 primary benchmarks, range [-0.5%, 0.2%]. |
| **Workload** | foldhash suite as run by rapidhash: hash-only, lookups and set build on 16 key distributions; rustc-perf compile-time benchmarks for the 2.0 switch. |
| **Metric** | time (ns per operation); instruction count for the rustc-perf run |
| **Hardware** | AMD EPYC 9R14 (rapidhash suite); rust-lang perf server (rustc-perf) |
| **Toolchain** | lto = "fat", codegen-units = 1, -C target-cpu=native (rapidhash suite) |
| **Measured** | 2025-08 |
| **Adoption** | 602 stars |
| **Last commit** | 2026-10-15 |
| **Platforms** | All Rust targets; no_std without the std feature |
| **Install** | cargo add rustc-hash |
| **Links** | [repository](https://github.com/rust-lang/rustc-hash) · [documentation](https://docs.rs/rustc-hash) · [benchmark](https://github.com/hoxxep/rapidhash/blob/master/docs/bench_hash_x86_64_amd_epyc_9R14_native.txt) |

## Verdict

On integer keys FxHash is the fastest hasher in independent tests, about 7x faster per lookup than SipHash-1-3. The published numbers are for the pre-2.0 algorithm, which collapses when keys differ only in high bits (7x slower than SipHash) and is slow on long inputs. The 2.0 rewrite targets those weaknesses, but its only published measurement is a -0.1% mean instruction-count change inside rustc.

**Why tier B:** Independent numbers from the rapidhash suite (code and hardware public) for the fxhash 0.2.1 crate, plus rust-lang's own rustc-perf run for the 2.0 update.

## Use it when

- Keys are trusted integers, pointers or small integer tuples, as in compilers and other tools with no attacker-controlled input.
- You want deterministic iteration order across runs (no random seed).

## Skip it when

- Any key can be chosen by an outside party: the hash is unkeyed and collisions can be precomputed.
- Keys are long strings or byte slices; seeded hashers such as foldhash or rapidhash are faster there.
- You are on pre-2.0 FxHash (rustc-hash 1.x or the fxhash crate) and your keys vary mainly in their high bits, such as multiples of large powers of two.

## How it is fast

Hashing an integer is one multiply (plus, since 2.0, an add and a final rotate); there is no per-map state, no seed and almost no finalization, so the whole hasher inlines. 2.0 replaced the Firefox-derived design with a polynomial hash, a bit rotation that moves high-entropy bits into the low bits hash tables index by, and a wyhash-style routine for strings.

## What it costs

- DoS resistance: none. The README states DoS attacks are not a concern for the compiler, which is why it uses a non-keyed hash.
- Pathological slowdowns are possible on structured keys even without an attacker.

## Problems with the evidence

- The independent per-operation numbers come from the old fxhash 0.2.1 crate, which uses the pre-2.0 algorithm; no independent per-operation numbers for rustc-hash 2.x were found.
- The rustc-perf result compares rustc-hash 2.0 with 1.x inside rustc, not with SipHash.
- The 2.0 rotation mitigates the high-bits collapse but the hash is still unkeyed.
- The separate fxhash crate (0.2.1, 2017), which still carries the old algorithm, has had no commits since 2019; rustc-hash 2 is its maintained successor.

## Evidence

<details>
<summary>7 quotes from the sources, checked 2026-10-28</summary>

> These attacks aren't a concern in the compiler so we prefer to use a quicker, non-cryptographic hash algorithm.
>
> — [github.com/rust-lang/rustc-hash/blob/main/README.md](https://github.com/rust-lang/rustc-hash/blob/main/README.md)

> fxhash = "0.2.1"
>
> — [github.com/hoxxep/rapidhash/blob/master/rapidhash-bench/Cargo.toml](https://github.com/hoxxep/rapidhash/blob/master/rapidhash-bench/Cargo.toml)

> │            u64 ┆  lookuphit ┆        2.21 ┆        2.73 ┆       2.21 ┆       2.71 ┆    2.89 ┆    2.03 ┆    2.81 ┆   14.74 │
>
> — [github.com/hoxxep/rapidhash/blob/master/docs/bench_hash_x86_64_amd_…](https://github.com/hoxxep/rapidhash/blob/master/docs/bench_hash_x86_64_amd_epyc_9R14_native.txt)

> │      u64hibits ┆   setbuild ┆        4.41 ┆        4.81 ┆       4.38 ┆       4.77 ┆    5.43 ┆   82.23 ┆    6.54 ┆   11.50 │
>
> — [github.com/hoxxep/rapidhash/blob/master/docs/bench_hash_x86_64_amd_…](https://github.com/hoxxep/rapidhash/blob/master/docs/bench_hash_x86_64_amd_epyc_9R14_native.txt)

> │       kilobyte ┆   hashonly ┆       41.63 ┆       42.45 ┆      47.21 ┆      47.29 ┆   19.33 ┆  156.68 ┆   33.07 ┆  226.41 │
>
> — [github.com/hoxxep/rapidhash/blob/master/docs/bench_hash_x86_64_amd_…](https://github.com/hoxxep/rapidhash/blob/master/docs/bench_hash_x86_64_amd_epyc_9R14_native.txt)

> All ❌✅ (primary) -0.1% [-0.5%, 0.2%] 18
>
> — [github.com/rust-lang/rust/pull/131949](https://github.com/rust-lang/rust/pull/131949)

> this means everything can collide for a hash table of size 2^n if all differences are in bits n or above.
>
> — [github.com/rust-lang/rustc-hash/pull/37](https://github.com/rust-lang/rustc-hash/pull/37)

</details>
