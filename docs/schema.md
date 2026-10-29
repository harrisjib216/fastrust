# Record schema

<sub>[fastrust](../README.md) › Docs</sub>

Each entry is one JSON file in [`data/entries/`](../data/entries). [`catalog render`](../tools) turns them into the
pages under [`catalog/`](../catalog) and two exports that carry every entry:

- [`catalog/catalog.jsonl`](../catalog/catalog.jsonl): one JSON object per line, sorted by `id`, with every field
  below plus `page`, the path of the entry's page.
- [`catalog/catalog.csv`](../catalog/catalog.csv): one row per entry with the summary columns listed at the end.

## Identity

| Field | Type | Meaning |
|---|---|---|
| `id` | string | The lowercase repository URL, with `#component` when one repository has two entries. Standard-library changes use `std:<module>#<component>` and toolchain changes `rustc:<feature>`. Crates without a repository use their crates.io page. |
| `name` | string | Display name, with the measured component in parentheses when it helps. |
| `domain`, `category` | string | Slugs from [`data/taxonomy.json`](../data/taxonomy.json). |

## The claim and the verdict

| Field | Type | Meaning |
|---|---|---|
| `baseline` | string | The default it claims to beat, as specifically as the source allows. |
| `claimed_speedup` | string | The published numbers. Anything computed from them is marked "(derived from page data)". |
| `workload` | string | What was measured. |
| `metric` | string | Throughput, latency, time, memory, build time and so on. |
| `hardware`, `os`, `toolchain` | string | CPU or machine, OS, and rustc version with flags (opt-level, LTO, `target-cpu`). Empty when the source doesn't say. |
| `benchmark_date` | string | `YYYY`, `YYYY-MM` or `YYYY-MM-DD`. Empty when unknown. |
| `benchmark_url` | string | The page the numbers come from. |
| `verdict` | `confirm` \| `downgrade` \| `reject` | Holds up as stated; holds up in part; doesn't hold up. |
| `verdict_reason` | string | Why, in a sentence or two. |
| `evidence_tier` | `A` \| `B` \| `C` \| `D` | A: public benchmark code with toolchain and hardware stated. B: independent of the authors. C: first-party. D: directional, or numbers only in images. |
| `tier_reason` | string | What put it in that tier. |
| `caveats` | string[] | Problems with the evidence. |
| `evidence` | object[] | `url` and `quote`: text copied exactly from the page, containing the numbers used. `catalog check` makes sure every number in `claimed_speedup` is in a quote, and `catalog verify` finds every quote on its page. |
| `checked` | string | The date the record was last checked against its sources. |

## The project

| Field | Type | Meaning |
|---|---|---|
| `repo_url`, `docs_url` | string or null | Repository, and the docs.rs page, std documentation, or the release notes or blog post for a toolchain change. |
| `license` | string or null | SPDX id from GitHub, or the crate's license expression from crates.io. |
| `usability` | `copyable` \| `reference` \| `platform` \| `mixed` | Open-source license; no open-source license (study only); ships with Rust (std, Cargo or rustc); mixed. |
| `language`, `platforms`, `install` | string | Implementation language, supported targets, and how to add it (`cargo add`, a profile setting, built into std since a version). |
| `maintenance_status` | string | `active` (a commit in the last year), `stale` (one to three years), `abandoned` (archived or older), `finished` or `unknown`, sometimes with a note. |
| `stars` | number or null | GitHub stars when checked. |
| `last_commit` | string | Date of the last commit seen. |

## The write-up

| Field | Type | Meaning |
|---|---|---|
| `one_liner` | string | What it is and what it beats, in one sentence. |
| `how_it_is_fast` | string | The mechanism. |
| `use_it_when`, `skip_it_when` | string[] | Where the win holds, and where it doesn't or isn't safe. |
| `tradeoffs` | string[] | What the speed costs. |

## catalog.csv

`id, name, domain, category, evidence_tier, verdict, usability, license, language, status, stars, last_commit,
repo_url, benchmark_url, baseline, one_liner, page`

## Examples

```sh
# Entries that hold up, with reproducible benchmarks
jq -c 'select(.verdict == "confirm" and .evidence_tier == "A") | {name, baseline, page}' catalog/catalog.jsonl

# What the standard library and toolchain absorbed
jq -r 'select(.usability == "platform") | "\(.benchmark_date) \(.name)"' catalog/catalog.jsonl | sort -r

# Count entries per tier
jq -r .evidence_tier catalog/catalog.jsonl | sort | uniq -c
```
