# Data parallelism

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Concurrency and runtime](../README.md)</sub>

**The default:** std::thread with hand-split work.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**rayon (work-stealing data parallelism)**](rayon-rs-rayon.md) | Work-stealing parallel iterators and join; its published speedups are against sequential code, not against hand-split std::thread. | C | 🟡 Holds up in part |
