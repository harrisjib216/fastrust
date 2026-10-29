# sold (mold fork for macOS)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Build and tooling](../README.md) › [Linkers](README.md)</sub>

**Tier D** (directional only) · ❌ Doesn't hold up · Open source · `C++` · `MIT` · 🔴 abandoned

> Archived macOS fork of mold; its author now points users to Apple's Xcode 15+ linker, which is already the default.

| | |
|---|---|
| **Beats** | Apple's default macOS linker called through cc: ld-prime with Xcode 15 or later, ld64 before that |
| **Claimed result** | No published numbers. The README called it a high-performance macOS linker; Bevy reports it is about the same speed as the default linker |
| **Workload** | None published: linking on macOS, compared only in prose |
| **Metric** | time (link) |
| **OS** | macOS |
| **Adoption** | 526 stars |
| **Last commit** | 2024-11-25 |
| **Platforms** | macOS and iOS (Mach-O) plus Linux (ELF, same as mold) |
| **Install** | Build from source; the repository is archived |
| **Links** | [repository](https://github.com/bluewhalesystems/sold) · [benchmark](https://github.com/bluewhalesystems/sold/blob/main/README.md) |

## Verdict

The author archived sold in November 2024 and now recommends Apple's new linker in Xcode 15 or later instead. Bevy's build guide says it is about the same speed as the default macOS linker. No published numbers show a gain over today's default.

**Why tier D:** No benchmark numbers were ever published; only the author's and Bevy's statements.

## Use it when

- None for new work: on Xcode 15 or later the default linker is the author's own recommendation

## Skip it when

- Any macOS build with Xcode 15 or later
- Any project that needs a maintained linker: the repository is archived

## How it is fast

sold carried mold's parallel linker design over to Mach-O at a time when Apple's ld64 was slow. Apple has since shipped a new, faster linker (ld-prime) with Xcode 15, which removed the reason for sold to exist.

## What it costs

- Unmaintained: no fixes for new Xcode or macOS releases

## Problems with the evidence

- The README never published benchmark numbers
- Bevy's comparison gives no figures

## Evidence

<details>
<summary>2 quotes from the sources, checked 2026-10-28</summary>

> is a high-performance linker for macOS. I built it because Apple's ld was too slow at the time, and I wasn't aware that they were developing a new linker. Xcode 15 or later are shipped with Apple's new faster linker. I recommend using that instead.
>
> — [github.com/bluewhalesystems/sold/blob/main/README.md](https://github.com/bluewhalesystems/sold/blob/main/README.md)

> There is a fork of Mold named Sold that supports macOS, but it is unmaintained and is about the same speed as # the default ld64 linker.
>
> — [github.com/bevyengine/bevy/blob/main/.cargo/config_fast_builds.toml](https://github.com/bevyengine/bevy/blob/main/.cargo/config_fast_builds.toml)

</details>
