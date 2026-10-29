# actix-web 4

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Server and networking](../README.md) › [Web frameworks](README.md)</sub>

**Tier A** (reproducible public benchmark) · 🟡 Holds up in part · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Mature Rust web framework that beats default-config axum 1.1-2.6x on small-request benchmarks; the lead shrinks to ~15% once a database is involved.

| | |
|---|---|
| **Beats** | axum with Tokio's default multi-threaded runtime: TechEmpower Round 22 entry (axum 0.6.16, hyper 0.14) and HttpArena 2026 standard-mode entry (axum 0.8) |
| **Claimed result** | TechEmpower Round 22 peak total requests per 15 s, actix vs axum: plaintext 64827336 vs 57504288, JSON 17109800 vs 11657647, fortunes (actix-web-pg-deadpool vs axum-pg-pool) 1458364 vs 1269748 (1.13x, 1.47x, 1.15x) (derived from page data). HttpArena, req/s actix vs axum: baseline-4096 2775120 vs 1080638, json-tls-4096 1157219 vs 967831, json-comp-4096 541211 vs 268142, pipelined-4096 16338425 vs 6474608 (2.57x, 1.20x, 2.02x, 2.52x) (derived from page data) |
| **Workload** | TechEmpower plaintext (pipelined), JSON and fortunes (Postgres + HTML template), peak req/s over 15-second runs. HttpArena: mixed GET/POST with query parsing, JSON over TLS, JSON with gzip/brotli, and 16x pipelined requests, all at 4,096 connections. |
| **Metric** | throughput |
| **Hardware** | TechEmpower Round 22 (Citrine): three machines with 28 HT cores Intel Xeon Gold 5120 @ 2.20GHz, 32 GB, switched 10-gigabit Ethernet. HttpArena: one AMD Threadripper PRO 3995WX workstation (64 cores). |
| **OS** | Linux, in Docker |
| **Toolchain** | Round 22: actix-web 4.3.1, RUSTFLAGS="-C target-cpu=native", lto, codegen-units 1; axum 0.6.16 on rust:1.67, target-cpu=native. HttpArena: actix-web 4 from git and axum 0.8 on rust:1.94, opt-level 3, codegen-units 1, lto thin. |
| **Measured** | 2026-09 |
| **Adoption** | 24.9k stars |
| **Last commit** | 2026-10-28 |
| **Platforms** | Linux, macOS, Windows |
| **Install** | cargo add actix-web |
| **Links** | [repository](https://github.com/actix/actix-web) · [documentation](https://docs.rs/actix-web) · [benchmark](https://www.http-arena.com/) |

## Verdict

actix-web is faster than default-config axum in both benchmarks, 1.13x to 2.57x on trivial handlers. With a database in the path (TechEmpower fortunes, both on deadpool-postgres) the lead is 15%, and actix failed to start in Round 23, so there is no comparison against the thread-per-core axum that caught up with ntex and xitca-web.

**Why tier A:** Two public benchmarks with published implementation code and stated hardware: TechEmpower Round 22 and HttpArena.

## Use it when

- You serve high rates of small requests and do not want to hand-tune Tokio for thread-per-core.
- You want a mature framework with a large ecosystem that runs thread-per-core out of the box.

## Skip it when

- Your requests are dominated by database or downstream latency; TechEmpower fortunes shows a 15% gap.
- You need tower middleware or hyper-based crates, which do not plug into actix-web.

## How it is fast

actix-web starts one single-threaded Tokio runtime per worker (one per core by default) and pins each connection to its worker, so there is no work-stealing or cross-thread synchronization per request. Its HTTP/1 codec writes into reused buffers.

## What it costs

- Not compatible with tower or hyper middleware.
- Thread-per-core scheduling can leave cores idle when connections carry uneven load.
- Each worker is single-threaded, so a CPU-heavy handler stalls every connection pinned to that worker.

## Problems with the evidence

- All actix entries failed to start in TechEmpower Round 23 ("ERROR: Problem starting actix"), so the latest TechEmpower comparison is Round 22 from October 2023, against axum 0.6 on hyper 0.14.
- In Round 23 axum moved to one current-thread runtime per core and came within 2-14% of ntex, xitca-web and may-minihttp on JSON; actix-web's lead over a tuned axum is unmeasured.
- The Round 22 fortunes comparison uses the deadpool-pooled entries of both frameworks; their unpooled entries (actix-http, a lower-level crate, vs axum-pg) favored axum 6584951 to 6077171 total requests.
- HttpArena runs the server and the load generator on the same workstation with pinned cores (loopback networking), and each framework's results are committed separately.
- TechEmpower plaintext and HttpArena pipelined use HTTP pipelining, which real clients rarely use.

## Evidence

<details>
<summary>23 quotes from the sources, checked 2026-10-28</summary>

> "actix": [ { "latencyAvg": "514.16us", "latencyStdev": "261.67us", "latencyMax": "5.41ms", "totalRequests": 64827336
>
> — [techempower.com/benchmarks/results/round22/ph.json](https://www.techempower.com/benchmarks/results/round22/ph.json)

> "axum": [ { "latencyAvg": "637.44us", "latencyStdev": "364.39us", "latencyMax": "13.47ms", "totalRequests": 56247808, "startTime": 1697063215, "endTime": 1697063230 }, { "latencyAvg": "2.97ms", "latencyStdev": "2.17ms", "latencyMax": "27.48ms", "totalRequests": 57504288
>
> — [techempower.com/benchmarks/results/round22/ph.json](https://www.techempower.com/benchmarks/results/round22/ph.json)

> "actix": [ { "latencyAvg": "106.13us", "latencyStdev": "18.57us", "latencyMax": "4.09ms", "totalRequests": 2178385, "startTime": 1697018821, "endTime": 1697018836 }, { "latencyAvg": "90.84us", "latencyStdev": "10.31us", "latencyMax": "2.08ms", "totalRequests": 4472469, "startTime": 1697018838, "endTime": 1697018853 }, { "latencyAvg": "97.33us", "latencyStdev": "55.97us", "latencyMax": "10.15ms", "…
>
> — [techempower.com/benchmarks/results/round22/ph.json](https://www.techempower.com/benchmarks/results/round22/ph.json)

> "axum": [ { "latencyAvg": "142.47us", "latencyStdev": "29.12us", "latencyMax": "3.07ms", "totalRequests": 1626056, "startTime": 1697063082, "endTime": 1697063097 }, { "latencyAvg": "156.53us", "latencyStdev": "29.25us", "latencyMax": "1.95ms", "totalRequests": 2605570, "startTime": 1697063099, "endTime": 1697063114 }, { "latencyAvg": "251.76us", "latencyStdev": "64.42us", "latencyMax": "1.91ms", "…
>
> — [techempower.com/benchmarks/results/round22/ph.json](https://www.techempower.com/benchmarks/results/round22/ph.json)

> "actix-web-pg-deadpool": [ { "latencyAvg": "804.62us", "latencyStdev": "72.24us", "latencyMax": "2.60ms", "totalRequests": 296870, "startTime": 1697023504, "endTime": 1697023519 }, { "latencyAvg": "575.24us", "latencyStdev": "88.51us", "latencyMax": "3.29ms", "totalRequests": 724721, "startTime": 1697023521, "endTime": 1697023536 }, { "latencyAvg": "570.71us", "latencyStdev": "102.30us", "latencyM…
>
> — [techempower.com/benchmarks/results/round22/ph.json](https://www.techempower.com/benchmarks/results/round22/ph.json)

> "axum-pg-pool": [ { "latencyAvg": "425.34us", "latencyStdev": "41.23us", "latencyMax": "1.01ms", "totalRequests": 557868, "startTime": 1697068030, "endTime": 1697068045 }, { "latencyAvg": "466.70us", "latencyStdev": "51.75us", "latencyMax": "4.56ms", "totalRequests": 890268, "startTime": 1697068047, "endTime": 1697068062 }, { "latencyAvg": "656.66us", "latencyStdev": "64.80us", "latencyMax": "3.10…
>
> — [techempower.com/benchmarks/results/round22/ph.json](https://www.techempower.com/benchmarks/results/round22/ph.json)

> "actix-http": [ { "latencyAvg": "363.49us", "latencyStdev": "24.42us", "latencyMax": "1.40ms", "totalRequests": 650817, "startTime": 1697019910, "endTime": 1697019925 }, { "latencyAvg": "369.28us", "latencyStdev": "36.51us", "latencyMax": "0.91ms", "totalRequests": 1118773, "startTime": 1697019927, "endTime": 1697019942 }, { "latencyAvg": "248.39us", "latencyStdev": "35.45us", "latencyMax": "1.27m…
>
> — [techempower.com/benchmarks/results/round22/ph.json](https://www.techempower.com/benchmarks/results/round22/ph.json)

> "axum-pg": [ { "latencyAvg": "376.88us", "latencyStdev": "33.82us", "latencyMax": "7.12ms", "totalRequests": 628321, "startTime": 1697066808, "endTime": 1697066823 }, { "latencyAvg": "387.59us", "latencyStdev": "46.55us", "latencyMax": "4.64ms", "totalRequests": 1067318, "startTime": 1697066825, "endTime": 1697066840 }, { "latencyAvg": "283.61us", "latencyStdev": "45.98us", "latencyMax": "8.30ms",…
>
> — [techempower.com/benchmarks/results/round22/ph.json](https://www.techempower.com/benchmarks/results/round22/ph.json)

> "name": "Continuous Benchmarking Run 2023-10-11 08:00:48"
>
> — [techempower.com/benchmarks/results/round22/ph.json](https://www.techempower.com/benchmarks/results/round22/ph.json)

> "actix": "ERROR: Problem starting actix"
>
> — [techempower.com/benchmarks/results/round23/ph.json](https://www.techempower.com/benchmarks/results/round23/ph.json)

> 28 HT Cores Intel(R) Xeon(R) Gold 5120 CPU @ 2.20GHz
>
> — [github.com/TechEmpower/FrameworkBenchmarks/wiki/Project-Information…](https://github.com/TechEmpower/FrameworkBenchmarks/wiki/Project-Information-Environment)

> Requests will be sent using HTTP pipelining.
>
> — [github.com/TechEmpower/FrameworkBenchmarks/wiki/Project-Information…](https://github.com/TechEmpower/FrameworkBenchmarks/wiki/Project-Information-Framework-Tests-Overview)

> "baseline-4096": { "framework": "actix", "language": "Rust", "rps": 2775120,
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/site/data/results/actix.json)

> "baseline-4096": { "framework": "axum", "language": "Rust", "rps": 1080638,
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/site/data/results/axum.json)

> "json-tls-4096": { "framework": "actix", "language": "Rust", "rps": 1157219,
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/site/data/results/actix.json)

> "json-tls-4096": { "framework": "axum", "language": "Rust", "rps": 967831,
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/site/data/results/axum.json)

> "json-comp-4096": { "framework": "actix", "language": "Rust", "rps": 541211,
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/site/data/results/actix.json)

> "json-comp-4096": { "framework": "axum", "language": "Rust", "rps": 268142,
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/site/data/results/axum.json)

> "pipelined-4096": { "framework": "actix", "language": "Rust", "rps": 16338425,
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/site/data/results/actix.json)

> "pipelined-4096": { "framework": "axum", "language": "Rust", "rps": 6474608,
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/site/data/results/axum.json)

> Axum 0.8 on hyper with the multi-threaded Tokio runtime, default configuration.
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/frameworks/axum/meta.json)

> The reference hardware for HttpArena results is a single-socket AMD Threadripper PRO 3995WX workstation.
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/site/content/docs/hardware/_index.md)

> Actix Web is a powerful, pragmatic, and extremely fast web framework for Rust
>
> — [github.com/actix/actix-web](https://github.com/actix/actix-web)

</details>
