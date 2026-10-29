# Build settings for faster binaries

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Build and tooling](../README.md)</sub>

**The default:** cargo build --release with default profile settings.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**cargo-pgo (profile-guided optimization)**](kobzol-cargo-pgo.md) | Cargo subcommand that automates profile-guided optimization; PGO made rustc itself 10-20% faster in several measurements. | B | ✅ Holds up |
| [**-C target-cpu=native**](rustc-target-cpu-native.md) | Compiling for the build machine's exact CPU helps vectorizable loops; for general code measured gains were nil or negative. | B | 🟡 Holds up in part |
| [**BOLT post-link optimization (via cargo-pgo)**](kobzol-cargo-pgo-bolt.md) | Profile-driven code layout applied after linking; adds roughly 2-6% on top of PGO in rustc measurements, at a size cost. | B | 🟡 Holds up in part |
| [**codegen-units = 1 in the release profile**](rustc-codegen-units-1.md) | Compiling each crate as one LLVM unit gains about 1.5-2% runtime and shrinks binaries, but roughly doubles release build time. | B | 🟡 Holds up in part |
| [**Fat LTO (lto = "fat") in the release profile**](rustc-lto-fat.md) | Whole-program LTO across all crates: single-digit runtime gains in the best measurement, at several times the release build time. | B | 🟡 Holds up in part |
| [**panic = "abort" in the release profile**](rustc-panic-abort.md) | Aborting instead of unwinding on panic shrinks binaries; any runtime gain is described as slight and is unmeasured. | D | 🟡 Holds up in part |
