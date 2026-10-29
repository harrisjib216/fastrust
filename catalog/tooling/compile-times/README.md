# Compile times

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Build and tooling](../README.md)</sub>

**The default:** cargo build with the LLVM backend and default profile settings.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**Less debug info in dev builds (debug = false or "line-tables-only")**](rustc-dev-debuginfo-off.md) | Turning off or trimming debug info in Cargo's dev profile cuts incremental rebuild times by up to 30-40%. | B | ✅ Holds up |
| [**PGO, LTO and BOLT in the official rustc builds**](rustc-pgo-dist-builds.md) | The rustc you install with rustup is already built with PGO and LTO (and BOLT on x86_64 Linux), making it 10-30% faster than a plain build. | B | ✅ Holds up |
| [**Pipelined compilation in Cargo (Rust 1.38)**](rustc-pipelined-compilation.md) | Since Rust 1.38, Cargo starts compiling a crate as soon as its dependencies' metadata is ready, not after they finish. | B | ✅ Holds up |
| [**Parallel rustc front end (-Z threads)**](rustc-parallel-front-end.md) | Nightly option that runs rustc's parsing, type checking and borrow checking on several threads; up to 50% faster builds. | B | 🟡 Holds up in part |
| [**rustc_codegen_cranelift (Cranelift codegen backend)**](rust-lang-rustc-codegen-cranelift.md) | Nightly-only codegen backend that compiles debug builds 5-20% faster than LLVM by generating less optimized code. | B | 🟡 Holds up in part |
