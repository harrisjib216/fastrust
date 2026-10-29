# cargo-nextest (test runner)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Build and tooling](../README.md) › [Test runners and build caching](README.md)</sub>

**Tier A** (reproducible public benchmark) · ✅ Holds up · Open source · `Rust` · `Apache-2.0 OR MIT` · 🟢 active

> Test runner that runs every test in its own process from one global pool; 1.4x-3.4x faster than cargo test on nine real projects.

| | |
|---|---|
| **Beats** | cargo test (libtest), which runs test binaries one after another |
| **Claimed result** | 1.37x to 3.38x faster test runs: crucible 5.14 s vs 1.52 s (3.38x), tokio 24.27 s vs 11.60 s (2.09x), omicron 444.08 s vs 202.50 s (2.19x), penumbra 125.38 s vs 90.96 s (1.37x) |
| **Workload** | Test execution only (build excluded) for 9 workspaces with 113 to 1138 tests each; minimum of runs 3-5; doctests excluded |
| **Metric** | time |
| **Hardware** | AMD Ryzen 9 7950X, 16 cores/32 threads, 64 GB RAM |
| **OS** | Pop!_OS 22.04, Linux kernel 6.0.12 |
| **Toolchain** | Rust 1.66.0 |
| **Adoption** | 3.4k stars |
| **Last commit** | 2026-10-28 |
| **Platforms** | Linux, macOS and Windows |
| **Install** | cargo install --locked cargo-nextest (or a pre-built binary) |
| **Links** | [repository](https://github.com/nextest-rs/nextest) · [documentation](https://nexte.st/) · [benchmark](https://nexte.st/docs/benchmarks/) |

## Verdict

On nine real projects pinned to specific commits, nextest ran the same tests 1.37x to 3.38x faster than cargo test, with commands and hardware published. The gain comes from running tests from different binaries in parallel, so it shrinks for projects with a single test binary.

**Why tier A:** Projects, revisions, commands, hardware and Rust version are all published, so the runs can be repeated; the numbers are first-party.

## Use it when

- Workspaces with many crates and test binaries
- Suites where a few long-running tests sit in different binaries
- CI pipelines where test time is a large share of the total

## Skip it when

- Doctests: nextest does not run them, so keep cargo test --doc
- Tests that depend on sharing in-process state with other tests in the same binary
- Small single-binary projects, where the project itself says runs may not get faster

## How it is fast

cargo test runs each test binary in turn, so a slow binary holds up all the others. nextest lists the tests in every binary first, then runs each test as its own process from a single shared pool, so slow tests in different binaries overlap.

## What it costs

- One process per test adds per-test startup cost
- Separate install; doctests need a second command

## Problems with the evidence

- First-party benchmark
- Measured with Rust 1.66.0, released December 2022
- Excludes build time, which often dominates CI runs

## Evidence

<details>
<summary>7 quotes from the sources, checked 2026-10-28</summary>

> | [crucible]    | [`cb228c2b`][crucible-rev]    |        483 |           5.14 |        1.52 |       3.38× |
>
> — [github.com/nextest-rs/nextest/blob/main/site/src/docs/benchmarks/in…](https://github.com/nextest-rs/nextest/blob/main/site/src/docs/benchmarks/index.md)

> | [tokio]       | [`1f50c571`][tokio-rev]       |       1138 |          24.27 |       11.60 |       2.09× |
>
> — [github.com/nextest-rs/nextest/blob/main/site/src/docs/benchmarks/in…](https://github.com/nextest-rs/nextest/blob/main/site/src/docs/benchmarks/index.md)

> | [omicron]     | [`e7949cd1`][omicron-rev]     |        619 |         444.08 |      202.50 |       2.19× |
>
> — [github.com/nextest-rs/nextest/blob/main/site/src/docs/benchmarks/in…](https://github.com/nextest-rs/nextest/blob/main/site/src/docs/benchmarks/index.md)

> | [penumbra]    | [`4ecd94cc`][penumbra-rev]    |        144 |         125.38 |       90.96 |       1.37× |
>
> — [github.com/nextest-rs/nextest/blob/main/site/src/docs/benchmarks/in…](https://github.com/nextest-rs/nextest/blob/main/site/src/docs/benchmarks/index.md)

> AMD Ryzen 9 7950X x86_64, 16 cores/32 threads
>
> — [github.com/nextest-rs/nextest/blob/main/site/src/docs/benchmarks/in…](https://github.com/nextest-rs/nextest/blob/main/site/src/docs/benchmarks/index.md)

> The measurements do not include time taken to build the tests. To ensure that, each command was run 5 times in succession. The measurement recorded is the minimum of runs 3, 4 and 5.
>
> — [github.com/nextest-rs/nextest/blob/main/site/src/docs/benchmarks/in…](https://github.com/nextest-rs/nextest/blob/main/site/src/docs/benchmarks/index.md)

> Even if nextest doesn't result in faster test runs, you may find it useful for identifying test bottlenecks, for its user interface, or for its other features.
>
> — [github.com/nextest-rs/nextest/blob/main/site/src/docs/benchmarks/in…](https://github.com/nextest-rs/nextest/blob/main/site/src/docs/benchmarks/index.md)

</details>
