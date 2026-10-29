# Build and tooling

<sub>[fastrust](../../README.md) › [Catalog](../README.md)</sub>

## [Linkers](linkers/README.md)

**The default:** The platform's default linker (GNU ld or BFD through cc on Linux, ld64 on macOS, link.exe on Windows).

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**mold (ELF linker)**](linkers/rui314-mold.md) | Parallel ELF linker by lld's original author; links large programs several times faster than lld and far faster than GNU ld. | A | ✅ Holds up |
| [**rust-lld as the default linker on x86_64 Linux (Rust 1.90)**](linkers/rustc-lld-default-linux.md) | Since Rust 1.90, rustc links x86_64 Linux programs with its bundled lld instead of GNU ld, which cuts link time several-fold. | B | ✅ Holds up |
| [**wild (ELF linker)**](linkers/wild-linker-wild.md) | Young Rust-written ELF linker aimed at fast iterative builds; beats lld where it works but trails mold and cannot link some large programs. | A | 🟡 Holds up in part |
| [**LLVM lld for macOS (ld64.lld)**](linkers/llvm-llvm-project-lld-macho.md) | LLVM's Mach-O linker: once 4x faster than Apple's ld64, now slower than Apple's ld-prime default on Apple silicon. | B | ❌ Doesn't hold up |
| [**sold (mold fork for macOS)**](linkers/bluewhalesystems-sold.md) | Archived macOS fork of mold; its author now points users to Apple's Xcode 15+ linker, which is already the default. | D | ❌ Doesn't hold up |
| [**zld (faster fork of Apple ld64)**](linkers/michaeleisel-zld.md) | Archived fork of Apple's old ld64 that claimed links at least 40% faster; Xcode 15's ld-prime replaced the linker it improved on. | D | ❌ Doesn't hold up |

## [Compile times](compile-times/README.md)

**The default:** cargo build with the LLVM backend and default profile settings.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**Less debug info in dev builds (debug = false or "line-tables-only")**](compile-times/rustc-dev-debuginfo-off.md) | Turning off or trimming debug info in Cargo's dev profile cuts incremental rebuild times by up to 30-40%. | B | ✅ Holds up |
| [**PGO, LTO and BOLT in the official rustc builds**](compile-times/rustc-pgo-dist-builds.md) | The rustc you install with rustup is already built with PGO and LTO (and BOLT on x86_64 Linux), making it 10-30% faster than a plain build. | B | ✅ Holds up |
| [**Pipelined compilation in Cargo (Rust 1.38)**](compile-times/rustc-pipelined-compilation.md) | Since Rust 1.38, Cargo starts compiling a crate as soon as its dependencies' metadata is ready, not after they finish. | B | ✅ Holds up |
| [**Parallel rustc front end (-Z threads)**](compile-times/rustc-parallel-front-end.md) | Nightly option that runs rustc's parsing, type checking and borrow checking on several threads; up to 50% faster builds. | B | 🟡 Holds up in part |
| [**rustc_codegen_cranelift (Cranelift codegen backend)**](compile-times/rust-lang-rustc-codegen-cranelift.md) | Nightly-only codegen backend that compiles debug builds 5-20% faster than LLVM by generating less optimized code. | B | 🟡 Holds up in part |

## [Build settings for faster binaries](runtime-build-settings/README.md)

**The default:** cargo build --release with default profile settings.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**cargo-pgo (profile-guided optimization)**](runtime-build-settings/kobzol-cargo-pgo.md) | Cargo subcommand that automates profile-guided optimization; PGO made rustc itself 10-20% faster in several measurements. | B | ✅ Holds up |
| [**-C target-cpu=native**](runtime-build-settings/rustc-target-cpu-native.md) | Compiling for the build machine's exact CPU helps vectorizable loops; for general code measured gains were nil or negative. | B | 🟡 Holds up in part |
| [**BOLT post-link optimization (via cargo-pgo)**](runtime-build-settings/kobzol-cargo-pgo-bolt.md) | Profile-driven code layout applied after linking; adds roughly 2-6% on top of PGO in rustc measurements, at a size cost. | B | 🟡 Holds up in part |
| [**codegen-units = 1 in the release profile**](runtime-build-settings/rustc-codegen-units-1.md) | Compiling each crate as one LLVM unit gains about 1.5-2% runtime and shrinks binaries, but roughly doubles release build time. | B | 🟡 Holds up in part |
| [**Fat LTO (lto = "fat") in the release profile**](runtime-build-settings/rustc-lto-fat.md) | Whole-program LTO across all crates: single-digit runtime gains in the best measurement, at several times the release build time. | B | 🟡 Holds up in part |
| [**panic = "abort" in the release profile**](runtime-build-settings/rustc-panic-abort.md) | Aborting instead of unwinding on panic shrinks binaries; any runtime gain is described as slight and is unmeasured. | D | 🟡 Holds up in part |

## [Test runners and build caching](test-runners-and-caching/README.md)

**The default:** cargo test, and no shared build cache.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**cargo-nextest (test runner)**](test-runners-and-caching/nextest-rs-nextest.md) | Test runner that runs every test in its own process from one global pool; 1.4x-3.4x faster than cargo test on nine real projects. | A | ✅ Holds up |
| [**sccache (shared compilation cache)**](test-runners-and-caching/mozilla-sccache.md) | Compiler-output cache that wraps rustc; helps clean CI builds of dependencies but cannot cache binaries, proc macros or incremental crates. | D | 🟡 Holds up in part |
