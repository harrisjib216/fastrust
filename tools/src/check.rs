use crate::model::{Catalog, Usability, Verdict};
use std::collections::{BTreeSet, HashMap};

/// Offline checks: every field present and well formed, every entry in a known category, every number in a claim
/// backed by a quote, no duplicate ids or slugs.
pub fn check(catalog: &Catalog) -> Vec<String> {
    let mut problems = Vec::new();
    let categories: BTreeSet<String> = catalog
        .domains
        .iter()
        .flat_map(|domain| domain.categories.iter().map(move |category| format!("{}/{}", domain.slug, category.slug)))
        .collect();
    let statuses = ["active", "stale", "abandoned", "finished", "unknown"];
    let mut ids: HashMap<&str, usize> = HashMap::new();
    let mut slugs: HashMap<String, usize> = HashMap::new();

    for entry in &catalog.entries {
        let slug = entry.slug();
        let mut problem = |text: String| problems.push(format!("{slug}: {text}"));
        *ids.entry(&entry.id).or_default() += 1;
        *slugs.entry(entry.slug()).or_default() += 1;

        if !categories.contains(&format!("{}/{}", entry.domain, entry.category)) {
            problem(format!("unknown category {}/{}", entry.domain, entry.category));
        }
        let required = [
            ("name", &entry.name),
            ("baseline", &entry.baseline),
            ("verdict_reason", &entry.verdict_reason),
            ("tier_reason", &entry.tier_reason),
            ("claimed_speedup", &entry.claimed_speedup),
            ("workload", &entry.workload),
            ("metric", &entry.metric),
            ("benchmark_url", &entry.benchmark_url),
            ("one_liner", &entry.one_liner),
            ("how_it_is_fast", &entry.how_it_is_fast),
            ("language", &entry.language),
            ("checked", &entry.checked),
        ];
        for (field, value) in required {
            if value.trim().is_empty() {
                problem(format!("{field} is empty"));
            }
        }
        let length = entry.one_liner.chars().count();
        if length > 200 {
            problem(format!("one_liner is {length} characters; keep it under 200"));
        }
        if entry.evidence.is_empty() {
            problem("no evidence".into());
        }
        for evidence in &entry.evidence {
            if !evidence.url.starts_with("https://") {
                problem(format!("evidence URL is not https: {}", evidence.url));
            }
        }
        if !entry.benchmark_date.is_empty() && !is_date(&entry.benchmark_date) {
            problem(format!("benchmark_date '{}' is not YYYY, YYYY-MM or YYYY-MM-DD", entry.benchmark_date));
        }
        if !entry.last_commit.is_empty() && !is_date(&entry.last_commit) {
            problem(format!("last_commit '{}' is not a date", entry.last_commit));
        }
        if !statuses.contains(&entry.status()) {
            problem(format!("maintenance_status '{}' is not one of {statuses:?}", entry.maintenance_status));
        }
        if entry.usability == Usability::Platform && entry.docs_url.is_none() {
            problem("a standard-library or toolchain entry needs docs_url".into());
        }
        if entry.use_it_when.is_empty() && entry.verdict != Verdict::Reject {
            problem("use_it_when is empty".into());
        }
        let quotes = entry.evidence.iter().map(|e| digits(&e.quote)).collect::<Vec<_>>().join(" ");
        for number in stated_numbers(&entry.claimed_speedup) {
            if !quotes.contains(&number) {
                problem(format!("{number} in claimed_speedup isn't in any quote; quote it or mark it derived"));
            }
        }
    }
    for (id, count) in ids {
        if count > 1 {
            problems.push(format!("id {id} is used {count} times"));
        }
    }
    for (slug, count) in slugs {
        if count > 1 {
            problems.push(format!("slug {slug} is used {count} times"));
        }
    }
    problems.sort();
    problems
}

/// `YYYY`, `YYYY-MM` or `YYYY-MM-DD`.
fn is_date(text: &str) -> bool {
    let parts: Vec<&str> = text.split('-').collect();
    let widths = [4, 2, 2];
    !parts.is_empty()
        && parts.len() <= 3
        && parts.iter().zip(widths).all(|(part, width)| part.len() == width && part.bytes().all(|b| b.is_ascii_digit()))
}

/// The numbers a claim states as measured, which must each be in a quote. Left out: ratios such as `2.6x` when the
/// claim says anything is derived, clauses and sentences marked "derived from page data" (but not "ratios derived",
/// which covers only the ratios), and version numbers such as `1.9.0`.
fn stated_numbers(claim: &str) -> Vec<String> {
    let lower = claim.to_lowercase();
    let anything_derived = lower.contains("derived");
    let clause_derived = |clause: &str| {
        let clause = clause.to_lowercase();
        clause.contains("derived") && !clause.contains("ratios derived") && !clause.contains("ratio derived")
    };
    claim
        .split(';')
        .flat_map(|clause| clause.split(". "))
        .filter(|clause| !clause_derived(clause))
        .flat_map(numbers)
        .filter(|(number, is_ratio)| !(*is_ratio && anything_derived) && number.matches('.').count() < 2)
        .map(|(number, _)| number)
        .collect()
}

/// Every number in `text`, and whether it's a ratio (followed by `x` or `×`). Numbers are runs of digits with the
/// decimal points and thousands separators inside them, written without separators so `1,234` matches `1234`.
fn numbers(text: &str) -> Vec<(String, bool)> {
    let chars: Vec<char> = text.chars().collect();
    let mut found = Vec::new();
    let mut index = 0;
    while index < chars.len() {
        if !chars[index].is_ascii_digit() {
            index += 1;
            continue;
        }
        let mut number = String::new();
        while index < chars.len() {
            let c = chars[index];
            let joins = (c == '.' || c == ',') && chars.get(index + 1).is_some_and(|n| n.is_ascii_digit());
            if c.is_ascii_digit() || c == '.' && joins {
                number.push(c);
            } else if !(c == ',' && joins) {
                break;
            }
            index += 1;
        }
        found.push((number, chars.get(index).is_some_and(|&c| c == 'x' || c == '×')));
    }
    found
}

/// The text with thousands separators between digits removed, for matching against `numbers`.
fn digits(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    chars
        .iter()
        .enumerate()
        .filter(|&(i, &c)| {
            !(c == ',' && i > 0 && chars[i - 1].is_ascii_digit() && chars.get(i + 1).is_some_and(|n| n.is_ascii_digit()))
        })
        .map(|(_, &c)| c)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dates() {
        assert!(is_date("2026"));
        assert!(is_date("2026-10"));
        assert!(is_date("2026-10-28"));
        assert!(!is_date("2026-1-28"));
        assert!(!is_date("Oct 2026"));
        assert!(!is_date(""));
    }

    #[test]
    fn numbers_in_a_claim() {
        let plain = |text| numbers(text).into_iter().map(|(n, _)| n).collect::<Vec<_>>();
        assert_eq!(plain("2.4x faster (1,310 ms vs 745 ms)."), ["2.4", "1310", "745"]);
        assert_eq!(plain("up to 50%, then 3"), ["50", "3"]);
        assert_eq!(stated_numbers("2x on Linux; 3.5 ms on macOS (derived from page data)"), Vec::<String>::new());
        assert_eq!(stated_numbers("2x on Linux, 30 ms vs 60 ms; 3.5 ms on macOS"), ["2", "30", "60", "3.5"]);
        assert_eq!(stated_numbers("30 vs 60 ms (2x) (ratios derived from page data)"), ["30", "60"]);
        assert_eq!(stated_numbers("regex 1.9.0 took 4 ms"), ["4"]);
        assert_eq!(stated_numbers("4 vs 8 ms. Per second: 250 vs 125 (derived from page data)"), ["4", "8"]);
        assert_eq!(digits("took 1,310 ms, then 2, 3"), "took 1310 ms, then 2, 3");
    }
}
