use crate::model::{Catalog, Category, Domain, Entry, Tier, Usability, Verdict};
use std::collections::BTreeMap;
use std::fs;

/// Writes catalog/ from scratch, plus the generated sections of the README.
pub fn render(catalog: &Catalog) -> Result<(), String> {
    let output = catalog.root.join("catalog");
    let _ = fs::remove_dir_all(&output);

    let mut files: BTreeMap<String, String> = BTreeMap::new();
    files.insert("README.md".into(), catalog_index(catalog));
    for (domain, categories) in catalog.used() {
        files.insert(format!("{}/README.md", domain.slug), domain_page(domain, &categories, catalog));
        for category in categories {
            let entries = catalog.entries_in(domain, category);
            files.insert(format!("{}/{}/README.md", domain.slug, category.slug), category_page(domain, category, &entries));
            for entry in entries {
                files.insert(
                    format!("{}/{}/{}.md", domain.slug, category.slug, entry.slug()),
                    entry_page(entry, domain, category),
                );
            }
        }
    }
    files.insert("catalog.jsonl".into(), json_lines(catalog)?);
    files.insert("catalog.csv".into(), csv(catalog));

    for (path, text) in files {
        let file = output.join(&path);
        if let Some(folder) = file.parent() {
            fs::create_dir_all(folder).map_err(|e| format!("{path}: {e}"))?;
        }
        fs::write(&file, text).map_err(|e| format!("{path}: {e}"))?;
    }
    update_readme(catalog)
}

// Pages

fn entry_page(entry: &Entry, domain: &Domain, category: &Category) -> String {
    let badges: Vec<String> = [
        Some(format!("**Tier {:?}** ({})", entry.evidence_tier, tier_label(entry.evidence_tier))),
        Some(verdict_label(entry.verdict).to_string()),
        Some(usability_label(entry.usability).to_string()),
        Some(format!("`{}`", entry.language)),
        entry.license.as_ref().map(|license| format!("`{license}`")),
        Some(status(entry)),
    ]
    .into_iter()
    .flatten()
    .collect();

    let mut lines = vec![
        format!("# {}", entry.name),
        String::new(),
        format!(
            "<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [{}](../README.md) › [{}](README.md)</sub>",
            domain.title, category.title
        ),
        String::new(),
        badges.join(" · "),
        String::new(),
        format!("> {}", entry.one_liner),
        String::new(),
        "| | |".into(),
        "|---|---|".into(),
    ];
    let rows = [
        ("Beats", entry.baseline.clone()),
        ("Claimed result", entry.claimed_speedup.clone()),
        ("Workload", entry.workload.clone()),
        ("Metric", entry.metric.clone()),
        ("Hardware", entry.hardware.clone()),
        ("OS", entry.os.clone()),
        ("Toolchain", entry.toolchain.clone()),
        ("Measured", entry.benchmark_date.clone()),
        ("Adoption", entry.stars.map(star_count).unwrap_or_default()),
        ("Last commit", entry.last_commit.clone()),
        ("Platforms", entry.platforms.clone()),
        ("Install", entry.install.clone()),
        ("Links", links(entry)),
    ];
    for (title, value) in rows {
        if !value.is_empty() {
            lines.push(format!("| **{title}** | {} |", cell(&value)));
        }
    }

    lines.extend([
        String::new(),
        "## Verdict".into(),
        String::new(),
        entry.verdict_reason.clone(),
        String::new(),
        format!("**Why tier {:?}:** {}", entry.evidence_tier, entry.tier_reason),
    ]);
    lines.extend(section("Use it when", &entry.use_it_when));
    lines.extend(section("Skip it when", &entry.skip_it_when));
    lines.extend([String::new(), "## How it is fast".into(), String::new(), entry.how_it_is_fast.clone()]);
    lines.extend(section("What it costs", &entry.tradeoffs));
    lines.extend(section("Problems with the evidence", &entry.caveats));

    let count = entry.evidence.len();
    lines.extend([
        String::new(),
        "## Evidence".into(),
        String::new(),
        "<details>".into(),
        format!(
            "<summary>{count} {} from the sources, checked {}</summary>",
            if count == 1 { "quote" } else { "quotes" },
            entry.checked
        ),
        String::new(),
    ]);
    for evidence in &entry.evidence {
        let mut quote: String = evidence.quote.chars().take(400).collect();
        if evidence.quote.chars().count() > 400 {
            quote.push('…');
        }
        lines.push(format!("> {}", without_relative_links(&quote).lines().collect::<Vec<_>>().join(" ")));
        lines.push(">".into());
        lines.push(format!("> — [{}]({})", short_url(&evidence.url), evidence.url));
        lines.push(String::new());
    }
    lines.push("</details>".into());
    lines.join("\n") + "\n"
}

fn category_page(domain: &Domain, category: &Category, entries: &[&Entry]) -> String {
    let mut lines = vec![
        format!("# {}", category.title),
        String::new(),
        format!("<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [{}](../README.md)</sub>", domain.title),
        String::new(),
        format!("**The default:** {}.", category.baseline),
        String::new(),
    ];
    lines.extend(entry_table(entries, ""));
    lines.join("\n") + "\n"
}

fn domain_page(domain: &Domain, categories: &[&Category], catalog: &Catalog) -> String {
    let mut lines = vec![
        format!("# {}", domain.title),
        String::new(),
        "<sub>[fastrust](../../README.md) › [Catalog](../README.md)</sub>".into(),
    ];
    for category in categories {
        lines.extend([
            String::new(),
            format!("## [{}]({}/README.md)", category.title, category.slug),
            String::new(),
            format!("**The default:** {}.", category.baseline),
            String::new(),
        ]);
        lines.extend(entry_table(&catalog.entries_in(domain, category), &format!("{}/", category.slug)));
    }
    lines.join("\n") + "\n"
}

fn catalog_index(catalog: &Catalog) -> String {
    let mut lines = vec![
        "# Catalog".to_string(),
        String::new(),
        "<sub>[fastrust](../README.md)</sub>".into(),
        String::new(),
        "Every entry, grouped by domain and category. Within a category, entries that hold up come first, then \
         partial ones, then rejected claims, each ordered by the strength of their evidence."
            .into(),
        String::new(),
    ];
    for (domain, categories) in catalog.used() {
        lines.push(format!("## [{}]({}/README.md)", domain.title, domain.slug));
        lines.push(String::new());
        for category in categories {
            let names: Vec<String> = catalog
                .entries_in(domain, category)
                .iter()
                .map(|entry| format!("[{}]({}/{}/{}.md)", entry.name, domain.slug, category.slug, entry.slug()))
                .collect();
            lines.push(format!(
                "- **[{}]({}/{}/README.md)**: {}",
                category.title,
                domain.slug,
                category.slug,
                names.join(", ")
            ));
        }
        lines.push(String::new());
    }
    lines.join("\n")
}

fn entry_table(entries: &[&Entry], link_prefix: &str) -> Vec<String> {
    let mut lines = vec!["| Entry | What it is | Tier | Verdict |".to_string(), "|---|---|:-:|---|".to_string()];
    for entry in entries {
        lines.push(format!(
            "| [**{}**]({link_prefix}{}.md) | {} | {:?} | {} |",
            cell(&entry.name),
            entry.slug(),
            cell(&entry.one_liner),
            entry.evidence_tier,
            verdict_label(entry.verdict)
        ));
    }
    lines
}

// README sections

fn update_readme(catalog: &Catalog) -> Result<(), String> {
    let path = catalog.root.join("README.md");
    let mut readme = fs::read_to_string(&path).map_err(|e| format!("README.md: {e}"))?;
    let entries = &catalog.entries;
    let count_tier = |tier: Tier| entries.iter().filter(|e| e.evidence_tier == tier).count();
    let count_verdict = |verdict: Verdict| entries.iter().filter(|e| e.verdict == verdict).count();
    let used = catalog.used();
    let category_count: usize = used.iter().map(|(_, categories)| categories.len()).sum();

    let badge = |label: &str, value: usize, color: &str| {
        format!(
            "  <img alt=\"{}: {value}\" src=\"https://img.shields.io/badge/{label}-{value}-{color}\">",
            label.replace('_', " ")
        )
    };
    let badges = [
        "<p align=\"center\">".to_string(),
        badge("entries", entries.len(), "2ea44f"),
        badge("tier_A", count_tier(Tier::A), "1f6feb"),
        badge("categories", category_count, "8250df"),
        badge("claims_rejected", count_verdict(Verdict::Reject), "cf222e"),
        "  <img alt=\"license: MIT\" src=\"https://img.shields.io/badge/license-MIT-blue\">".into(),
        "</p>".into(),
    ];
    readme = replace_section("badges", &readme, &badges.join("\n"));

    let tiers: Vec<String> =
        [Tier::A, Tier::B, Tier::C, Tier::D].iter().map(|&tier| format!("{} tier {tier:?}", count_tier(tier))).collect();
    let stats = format!(
        "**{} entries** across **{category_count} categories** in **{} domains** · {} · {} hold up · {} hold up in part · {} rejected",
        entries.len(),
        used.len(),
        tiers.join(" · "),
        count_verdict(Verdict::Confirm),
        count_verdict(Verdict::Downgrade),
        count_verdict(Verdict::Reject)
    );
    readme = replace_section("stats", &readme, &stats);

    let mut index = vec!["| Domain | Entries | Categories |".to_string(), "|---|--:|---|".to_string()];
    for (domain, categories) in &used {
        let links: Vec<String> = categories
            .iter()
            .map(|category| {
                format!(
                    "[{}](catalog/{}/{}/README.md) {}",
                    category.title,
                    domain.slug,
                    category.slug,
                    catalog.entries_in(domain, category).len()
                )
            })
            .collect();
        let count = entries.iter().filter(|e| e.domain == domain.slug).count();
        index.push(format!(
            "| [**{}**](catalog/{}/README.md) | {count} | {} |",
            domain.title,
            domain.slug,
            links.join(" · ")
        ));
    }
    readme = replace_section("index", &readme, &index.join("\n"));

    // The strongest entry in each domain that holds up as claimed.
    let mut featured = vec!["| Need faster… | Pick | What it is | Tier |".to_string(), "|---|---|---|:-:|".to_string()];
    for (domain, _) in &used {
        let best = entries
            .iter()
            .filter(|e| e.domain == domain.slug && e.verdict == Verdict::Confirm)
            .min_by_key(|e| (e.evidence_tier, std::cmp::Reverse(e.stars.unwrap_or(0)), e.name.to_lowercase()));
        let Some(best) = best else { continue };
        let Some(category) = domain.categories.iter().find(|c| c.slug == best.category) else { continue };
        let path = format!("catalog/{}/{}", domain.slug, category.slug);
        featured.push(format!(
            "| [{}]({path}/README.md) | [**{}**]({path}/{}.md) | {} | **{:?}** |",
            category.title,
            cell(&best.name),
            best.slug(),
            cell(&best.one_liner),
            best.evidence_tier
        ));
    }
    readme = replace_section("featured", &readme, &featured.join("\n"));

    let mut rejected: Vec<&Entry> = entries.iter().filter(|e| e.verdict == Verdict::Reject).collect();
    rejected.sort_by_key(|e| e.name.to_lowercase());
    let mut table = vec!["| Claim | Why it doesn't hold up |".to_string(), "|---|---|".to_string()];
    for entry in rejected {
        table.push(format!("| [**{}**]({}) | {} |", cell(&entry.name), entry.page(), cell(&entry.verdict_reason)));
    }
    readme = replace_section("rejected", &readme, &table.join("\n"));

    // The changes Rust shipped to its own defaults, newest release first: entries named for the version that brought
    // them, such as "(Rust 1.67)".
    let mut absorbed: Vec<(u32, &Entry)> = entries
        .iter()
        .filter(|e| e.usability == Usability::Platform)
        .filter_map(|e| rust_version(&e.name).map(|minor| (minor, e)))
        .collect();
    absorbed.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.name.cmp(&b.1.name)));
    let mut caught_up = vec!["| Release | Change | What it does | Tier |".to_string(), "|---|---|---|:-:|".to_string()];
    for (minor, entry) in absorbed {
        caught_up.push(format!(
            "| 1.{minor} | [**{}**]({}) | {} | {:?} |",
            cell(&entry.name),
            entry.page(),
            cell(&entry.one_liner),
            entry.evidence_tier
        ));
    }
    readme = replace_section("caught-up", &readme, &caught_up.join("\n"));

    fs::write(&path, readme).map_err(|e| format!("README.md: {e}"))
}

/// The minor version in a name like "std::sync::mpsc (Rust 1.67)".
fn rust_version(name: &str) -> Option<u32> {
    let start = name.find("Rust 1.")? + "Rust 1.".len();
    let digits: String = name[start..].chars().take_while(char::is_ascii_digit).collect();
    digits.parse().ok()
}

/// Replaces the text between `<!-- fastrust:name -->` and `<!-- /fastrust:name -->`.
fn replace_section(name: &str, text: &str, content: &str) -> String {
    let open = format!("<!-- fastrust:{name} -->");
    let close = format!("<!-- /fastrust:{name} -->");
    let Some(start) = text.find(&open).map(|i| i + open.len()) else { return text.to_string() };
    let Some(end) = text[start..].find(&close).map(|i| i + start) else { return text.to_string() };
    format!("{}\n{content}\n{}", &text[..start], &text[end..])
}

// Exports

fn json_lines(catalog: &Catalog) -> Result<String, String> {
    let mut entries: Vec<&Entry> = catalog.entries.iter().collect();
    entries.sort_by(|a, b| a.id.cmp(&b.id));
    let mut lines = Vec::with_capacity(entries.len());
    for entry in entries {
        // serde_json's Map keeps keys sorted, so every line has the same field order.
        let mut object = serde_json::to_value(entry).map_err(|e| e.to_string())?;
        object["page"] = entry.page().into();
        lines.push(serde_json::to_string(&object).map_err(|e| e.to_string())?);
    }
    Ok(lines.join("\n") + "\n")
}

fn csv(catalog: &Catalog) -> String {
    let header = [
        "id", "name", "domain", "category", "evidence_tier", "verdict", "usability", "license", "language", "status",
        "stars", "last_commit", "repo_url", "benchmark_url", "baseline", "one_liner", "page",
    ];
    let mut entries: Vec<&Entry> = catalog.entries.iter().collect();
    entries.sort_by(|a, b| a.id.cmp(&b.id));
    let mut rows = vec![header.join(",")];
    for entry in entries {
        let fields = [
            entry.id.clone(),
            entry.name.clone(),
            entry.domain.clone(),
            entry.category.clone(),
            format!("{:?}", entry.evidence_tier),
            format!("{:?}", entry.verdict).to_lowercase(),
            format!("{:?}", entry.usability).to_lowercase(),
            entry.license.clone().unwrap_or_default(),
            entry.language.clone(),
            entry.status().to_string(),
            entry.stars.map(|s| s.to_string()).unwrap_or_default(),
            entry.last_commit.clone(),
            entry.repo_url.clone().unwrap_or_default(),
            entry.benchmark_url.clone(),
            entry.baseline.clone(),
            entry.one_liner.clone(),
            entry.page(),
        ];
        let quoted: Vec<String> = fields
            .iter()
            .map(|field| {
                if field.contains([',', '"', '\n', '\r']) {
                    format!("\"{}\"", field.replace('"', "\"\""))
                } else {
                    field.clone()
                }
            })
            .collect();
        rows.push(quoted.join(","));
    }
    rows.join("\n") + "\n"
}

// Formatting

fn tier_label(tier: Tier) -> &'static str {
    match tier {
        Tier::A => "reproducible public benchmark",
        Tier::B => "independent measurement",
        Tier::C => "first-party numbers",
        Tier::D => "directional only",
    }
}

fn verdict_label(verdict: Verdict) -> &'static str {
    match verdict {
        Verdict::Confirm => "✅ Holds up",
        Verdict::Downgrade => "🟡 Holds up in part",
        Verdict::Reject => "❌ Doesn't hold up",
    }
}

fn usability_label(usability: Usability) -> &'static str {
    match usability {
        Usability::Copyable => "Open source",
        Usability::Reference => "No open-source license",
        Usability::Platform => "Ships with Rust",
        Usability::Mixed => "Mixed licensing",
    }
}

fn status(entry: &Entry) -> String {
    let word = if entry.status().is_empty() { "unknown" } else { entry.status() };
    let mark = match word {
        "active" => "🟢",
        "stale" => "🟡",
        "abandoned" => "🔴",
        _ => "⚪",
    };
    format!("{mark} {word}")
}

fn links(entry: &Entry) -> String {
    let mut links = Vec::new();
    if let Some(repo) = entry.repo_url.as_deref().filter(|r| !r.is_empty()) {
        links.push(format!("[repository]({repo})"));
    }
    if let Some(docs) = entry.docs_url.as_deref().filter(|d| !d.is_empty()) {
        links.push(format!("[documentation]({docs})"));
    }
    if !entry.benchmark_url.is_empty() {
        links.push(format!("[benchmark]({})", entry.benchmark_url));
    }
    links.join(" · ")
}

fn section(title: &str, items: &[String]) -> Vec<String> {
    if items.is_empty() {
        return Vec::new();
    }
    let mut lines = vec![String::new(), format!("## {title}"), String::new()];
    lines.extend(items.iter().map(|item| format!("- {item}")));
    lines
}

fn star_count(stars: u64) -> String {
    if stars >= 1000 { format!("{:.1}k stars", stars as f64 / 1000.0) } else { format!("{stars} stars") }
}

fn short_url(url: &str) -> String {
    let text = url.replace("https://", "").replace("www.", "");
    if text.chars().count() > 70 { text.chars().take(67).collect::<String>() + "…" } else { text }
}

/// Relative Markdown links in a quote point into the quoted page's repository, not this one, so they're shown as
/// their text. The quote in the record stays verbatim.
fn without_relative_links(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(open) = rest.find('[') {
        let after = &rest[open + 1..];
        let link = after.find("](").and_then(|close| {
            let label = &after[..close];
            let target = &after[close + 2..];
            let end = target.find(')')?;
            let relative = !target[..end].contains("://") && !target[..end].starts_with('#');
            (relative && !label.contains(['[', ']'])).then_some((label, &target[end + 1..]))
        });
        match link {
            Some((label, remainder)) => {
                result.push_str(&rest[..open]);
                result.push_str(label);
                rest = remainder;
            }
            None => {
                result.push_str(&rest[..=open]);
                rest = after;
            }
        }
    }
    result.push_str(rest);
    result
}

/// Table cells can't hold pipes or line breaks.
fn cell(text: &str) -> String {
    text.replace('|', "\\|").lines().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sections_are_replaced_between_their_markers() {
        let text = "a\n<!-- fastrust:stats -->\nold\n<!-- /fastrust:stats -->\nb";
        assert_eq!(replace_section("stats", text, "new"), "a\n<!-- fastrust:stats -->\nnew\n<!-- /fastrust:stats -->\nb");
        assert_eq!(replace_section("missing", text, "new"), text);
    }

    #[test]
    fn versions_in_names() {
        assert_eq!(rust_version("std::sync::mpsc (crossbeam-channel implementation, Rust 1.67)"), Some(67));
        assert_eq!(rust_version("rust-lld as the default linker on x86_64 Linux (Rust 1.90)"), Some(90));
        assert_eq!(rust_version("-C target-cpu=native"), None);
    }

    #[test]
    fn relative_links_in_quotes() {
        assert_eq!(
            without_relative_links("[pcre2](benchmarks/../engines/pcre2) beats [regex](https://x.y/regex) [a] b"),
            "pcre2 beats [regex](https://x.y/regex) [a] b"
        );
    }

    #[test]
    fn cells_and_counts() {
        assert_eq!(cell("a | b\nc"), "a \\| b c");
        assert_eq!(star_count(15_234), "15.2k stars");
        assert_eq!(star_count(999), "999 stars");
    }
}
