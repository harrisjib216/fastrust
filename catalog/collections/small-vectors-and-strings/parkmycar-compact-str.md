# compact_str (CompactString, 24-byte inline string)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Collections](../README.md) › [Small vectors and strings](README.md)</sub>

**Tier D** (directional only) · 🟡 Holds up in part · Open source · `Rust` · `MIT` · 🟢 active

> Mutable String replacement that stores strings up to 24 bytes inline in the same 24 bytes; avoids allocations for short strings.

| | |
|---|---|
| **Beats** | String for short strings |
| **Claimed result** | No numbers published in text. Stores up to 24 bytes inline in a value the same size as String. |
| **Workload** | string-rosetta-rs: creating, cloning, accessing and comparing strings of varying length (charts only). |
| **Metric** | memory; time (charts only) |
| **Measured** | 2026-03-09 |
| **Adoption** | 877 stars |
| **Last commit** | 2026-07-13 |
| **Platforms** | All Rust targets (12-byte inline limit on 32-bit); no_std |
| **Install** | cargo add compact_str |
| **Links** | [repository](https://github.com/ParkMyCar/compact_str) · [documentation](https://docs.rs/compact_str) · [benchmark](https://github.com/rosetta-rs/string-rosetta-rs) |

## Verdict

The memory property is certain: strings up to 24 bytes are stored inline in a type the size of String, so they need no allocation. The README's claim that this makes it more performant has no published numbers, and the third-party comparison it points to shows results only as charts.

**Why tier D:** No benchmark numbers in text from the project; the referenced string-rosetta-rs results are chart images only.

## Use it when

- Parsing or deserializing many short strings that you may later mutate.
- Option<CompactString> fields where the niche keeps the size at 24 bytes.

## Skip it when

- Strings are cloned far more than created: clone is O(n); an O(1)-clone type such as ecow or arcstr fits better.
- Profiling has not shown string allocation as a cost: the rosetta-rs maintainers suggest String by default.

## How it is fast

The last byte of the 24-byte value doubles as a discriminant and length, so up to 24 bytes of text fit inline with no heap pointer. Short-string creation and clone copy with two overlapping fixed-width loads and stores instead of calling memcpy, and accesses use branchless selection between inline and heap layouts.

## What it costs

- Uses unsafe code (tested with miri, proptest and fuzzing per string-rosetta-rs).
- Heap strings grow by 1.5x rather than 2x, which can mean more reallocations for strings built by repeated pushes.

## Problems with the evidence

- Speed claims are unquantified in text; string-rosetta-rs results (run 2026-03-09) are SVG charts.
- string-rosetta-rs notes one competitor (smartstring) was skipped in the access benchmark for being slow, which shows inline-string designs differ a lot; measure before switching.

## Evidence

<details>
<summary>3 quotes from the sources, checked 2026-10-28</summary>

> Stores up to 24 bytes on the stack
>
> — [github.com/ParkMyCar/compact_str/blob/main/compact_str/README.md](https://github.com/ParkMyCar/compact_str/blob/main/compact_str/README.md)

> don't have to heap allocate so it's more performant.
>
> — [github.com/ParkMyCar/compact_str/blob/main/compact_str/README.md](https://github.com/ParkMyCar/compact_str/blob/main/compact_str/README.md)

> *(`smartstring` is skipped due to how slow it is)*
>
> — [github.com/rosetta-rs/string-rosetta-rs/blob/master/README.md](https://github.com/rosetta-rs/string-rosetta-rs/blob/master/README.md)

</details>
