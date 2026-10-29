use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// One catalog entry: a crate, standard-library change or tool, the default it claims to beat, and how far the
/// claim holds. The fields are documented in docs/schema.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    pub id: String,
    pub name: String,
    pub domain: String,
    pub category: String,
    pub baseline: String,
    pub verdict: Verdict,
    pub verdict_reason: String,
    pub evidence_tier: Tier,
    pub tier_reason: String,
    pub claimed_speedup: String,
    pub workload: String,
    pub metric: String,
    pub hardware: String,
    pub os: String,
    pub toolchain: String,
    pub benchmark_date: String,
    pub benchmark_url: String,
    pub repo_url: Option<String>,
    pub docs_url: Option<String>,
    pub license: Option<String>,
    pub usability: Usability,
    pub language: String,
    pub platforms: String,
    pub install: String,
    pub maintenance_status: String,
    pub stars: Option<u64>,
    pub last_commit: String,
    pub one_liner: String,
    pub how_it_is_fast: String,
    pub use_it_when: Vec<String>,
    pub skip_it_when: Vec<String>,
    pub caveats: Vec<String>,
    pub tradeoffs: Vec<String>,
    pub evidence: Vec<Evidence>,
    pub checked: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    pub url: String,
    pub quote: String,
}

/// Declared in the order the catalog lists them: claims that hold up first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Verdict {
    Confirm,
    Downgrade,
    Reject,
}

/// Declared from the strongest evidence to the weakest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Tier {
    A,
    B,
    C,
    D,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Usability {
    Copyable,
    Reference,
    Platform,
    Mixed,
}

impl Entry {
    /// The file name and page name: `rust-lang-hashbrown`, `std-sync-mpsc`.
    pub fn slug(&self) -> String {
        let mut text = self.id.to_lowercase();
        for prefix in ["https://", "http://", "github.com/", "www.", "github.com/"] {
            if let Some(rest) = text.strip_prefix(prefix) {
                text = rest.to_string();
            }
        }
        text.split(|c: char| !c.is_alphanumeric())
            .filter(|piece| !piece.is_empty())
            .collect::<Vec<_>>()
            .join("-")
    }

    /// The first word of `maintenance_status`, which may carry a note after it.
    pub fn status(&self) -> &str {
        self.maintenance_status.split_whitespace().next().unwrap_or("")
    }

    /// Where the entry's page lives, relative to the repository root.
    pub fn page(&self) -> String {
        format!("catalog/{}/{}/{}.md", self.domain, self.category, self.slug())
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Domain {
    pub slug: String,
    pub title: String,
    pub categories: Vec<Category>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Category {
    pub slug: String,
    pub title: String,
    /// What a Rust developer reaches for before looking for something faster.
    #[serde(rename = "default")]
    pub baseline: String,
}

/// The records in data/ and the taxonomy that orders them.
pub struct Catalog {
    pub root: PathBuf,
    pub domains: Vec<Domain>,
    pub entries: Vec<Entry>,
}

impl Catalog {
    pub fn load(root: &Path) -> Result<Catalog, String> {
        let data = root.join("data");
        let taxonomy = fs::read_to_string(data.join("taxonomy.json")).map_err(|e| format!("data/taxonomy.json: {e}"))?;
        let domains: Vec<Domain> = serde_json::from_str(&taxonomy).map_err(|e| format!("data/taxonomy.json: {e}"))?;

        let mut files: Vec<PathBuf> = fs::read_dir(data.join("entries"))
            .map_err(|e| format!("data/entries: {e}"))?
            .filter_map(|item| item.ok().map(|item| item.path()))
            .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
            .collect();
        files.sort();

        let mut entries = Vec::with_capacity(files.len());
        for file in files {
            let name = file.file_name().unwrap_or_default().to_string_lossy().into_owned();
            let text = fs::read_to_string(&file).map_err(|e| format!("data/entries/{name}: {e}"))?;
            entries.push(serde_json::from_str(&text).map_err(|e| format!("data/entries/{name}: {e}"))?);
        }
        Ok(Catalog { root: root.to_path_buf(), domains, entries })
    }

    /// The entries in one category, in catalog order: claims that hold up first, then the strongest evidence, then
    /// by name.
    pub fn entries_in(&self, domain: &Domain, category: &Category) -> Vec<&Entry> {
        let mut entries: Vec<&Entry> =
            self.entries.iter().filter(|e| e.domain == domain.slug && e.category == category.slug).collect();
        entries.sort_by(|a, b| order(a, b));
        entries
    }

    /// The domains and categories that have at least one entry.
    pub fn used(&self) -> Vec<(&Domain, Vec<&Category>)> {
        self.domains
            .iter()
            .map(|domain| {
                let categories =
                    domain.categories.iter().filter(|category| !self.entries_in(domain, category).is_empty()).collect();
                (domain, categories)
            })
            .filter(|(_, categories): &(&Domain, Vec<&Category>)| !categories.is_empty())
            .collect()
    }
}

pub fn order(a: &Entry, b: &Entry) -> std::cmp::Ordering {
    (a.verdict, a.evidence_tier, a.name.to_lowercase()).cmp(&(b.verdict, b.evidence_tier, b.name.to_lowercase()))
}
