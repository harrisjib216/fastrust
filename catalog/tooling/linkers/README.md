# Linkers

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Build and tooling](../README.md)</sub>

**The default:** The platform's default linker (GNU ld or BFD through cc on Linux, ld64 on macOS, link.exe on Windows).

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**mold (ELF linker)**](rui314-mold.md) | Parallel ELF linker by lld's original author; links large programs several times faster than lld and far faster than GNU ld. | A | ✅ Holds up |
| [**rust-lld as the default linker on x86_64 Linux (Rust 1.90)**](rustc-lld-default-linux.md) | Since Rust 1.90, rustc links x86_64 Linux programs with its bundled lld instead of GNU ld, which cuts link time several-fold. | B | ✅ Holds up |
| [**wild (ELF linker)**](wild-linker-wild.md) | Young Rust-written ELF linker aimed at fast iterative builds; beats lld where it works but trails mold and cannot link some large programs. | A | 🟡 Holds up in part |
| [**LLVM lld for macOS (ld64.lld)**](llvm-llvm-project-lld-macho.md) | LLVM's Mach-O linker: once 4x faster than Apple's ld64, now slower than Apple's ld-prime default on Apple silicon. | B | ❌ Doesn't hold up |
| [**sold (mold fork for macOS)**](bluewhalesystems-sold.md) | Archived macOS fork of mold; its author now points users to Apple's Xcode 15+ linker, which is already the default. | D | ❌ Doesn't hold up |
| [**zld (faster fork of Apple ld64)**](michaeleisel-zld.md) | Archived fork of Apple's old ld64 that claimed links at least 40% faster; Xcode 15's ld-prime replaced the linker it improved on. | D | ❌ Doesn't hold up |
