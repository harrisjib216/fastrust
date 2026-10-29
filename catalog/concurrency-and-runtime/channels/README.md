# Channels

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Concurrency and runtime](../README.md)</sub>

**The default:** std::sync::mpsc.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**std::sync::mpsc (crossbeam-channel implementation, Rust 1.67)**](std-sync-mpsc.md) | Since Rust 1.67 std::sync::mpsc wraps a copy of crossbeam-channel, closing most of the gap that made crossbeam the default recommendation. | D | ✅ Holds up |
| [**crossbeam-channel**](crossbeam-rs-crossbeam-crossbeam-channel.md) | Multi-producer multi-consumer channels with select!; its speed lead over std::sync::mpsc ended when std adopted its implementation in Rust 1.67. | D | 🟡 Holds up in part |
| [**flume**](zesterer-flume.md) | MPMC channel with sync and async APIs and no unsafe code; its 'always faster than std mpsc' chart predates std adopting crossbeam's design. | D | 🟡 Holds up in part |
| [**kanal**](fereidani-kanal.md) | MPMC channel with sync and async APIs that leads most of its own 2025 charts against std mpsc and crossbeam, though not bounded SPSC. | D | 🟡 Holds up in part |
