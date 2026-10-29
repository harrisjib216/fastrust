# sccache (shared compilation cache)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Build and tooling](../README.md) › [Test runners and build caching](README.md)</sub>

**Tier D** (directional only) · 🟡 Holds up in part · Open source · `Rust` · `Apache-2.0` · 🟢 active

> Compiler-output cache that wraps rustc; helps clean CI builds of dependencies but cannot cache binaries, proc macros or incremental crates.

| | |
|---|---|
| **Beats** | cargo build with no shared cache: every machine or CI job recompiles every dependency |
| **Claimed result** | No published numbers; the README says it avoids compilation when possible |
| **Workload** | None published: rebuilding Rust crates with a shared cache |
| **Metric** | build time |
| **Adoption** | 7.9k stars |
| **Last commit** | 2026-10-26 |
| **Platforms** | Linux, macOS and Windows; local disk or several cloud storage backends |
| **Install** | cargo install sccache (or a prebuilt binary); set RUSTC_WRAPPER=sccache |
| **Links** | [repository](https://github.com/mozilla/sccache) · [documentation](https://github.com/mozilla/sccache/blob/main/docs/Rust.md) · [benchmark](https://github.com/mozilla/sccache/blob/main/README.md) |

## Verdict

sccache can skip recompiling crates it has seen before, but it publishes no Rust timing numbers, and its own docs exclude much of a Rust build from caching: crates that invoke the linker (bin, dylib, cdylib, proc-macro) and anything compiled incrementally. Expect gains on clean CI builds of dependencies, not on local incremental rebuilds.

**Why tier D:** No benchmark numbers from the project or from any source found.

## Use it when

- Ephemeral CI runners that rebuild the same dependencies on every job
- Teams sharing a cache backend across machines

## Skip it when

- Local incremental dev builds: incrementally compiled crates are not cached
- Workspaces dominated by bin, cdylib or proc-macro crates
- Slow network links to the cache backend

## How it is fast

sccache sits between Cargo and rustc as a compiler wrapper. When an invocation matches one it has seen, it returns the stored output instead of compiling; a shared backend lets CI jobs and developers reuse each other's dependency builds.

## What it costs

- Cached crates must be built without incremental compilation
- Storage and network cost for shared backends

## Problems with the evidence

- No published Rust numbers found from the project or third parties
- Hit rates depend on identical flags, paths and toolchains

## Evidence

<details>
<summary>3 quotes from the sources, checked 2026-10-28</summary>

> It is used as a compiler wrapper and avoids compilation when possible
>
> — [github.com/mozilla/sccache/blob/main/README.md](https://github.com/mozilla/sccache/blob/main/README.md)

> Crates that invoke the system linker cannot be cached. This includes `bin`, `dylib`, `cdylib`, and `proc-macro` crates.
>
> — [github.com/mozilla/sccache/blob/main/README.md](https://github.com/mozilla/sccache/blob/main/README.md)

> Incrementally compiled crates cannot be cached.
>
> — [github.com/mozilla/sccache/blob/main/README.md](https://github.com/mozilla/sccache/blob/main/README.md)

</details>
