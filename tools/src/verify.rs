use crate::model::Catalog;
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, mpsc};
use std::thread;
use std::time::Duration;

pub struct Report {
    pub failures: Vec<String>,
    pub unreachable: Vec<String>,
}

/// Re-fetches every evidence URL and checks that every quote appears on the page.
///
/// Matching ignores case and collapses whitespace. A page counts as containing a quote if any of its readings does:
/// the raw text, the text with HTML tags removed (joined with and without spaces, since a number split across two
/// tags reads either way), and for JSON, its string values. Pages are cached in .cache/ so reruns are fast.
pub fn verify(catalog: &Catalog, token: Option<&str>) -> Report {
    let cache = catalog.root.join(".cache");
    let _ = fs::create_dir_all(&cache);
    let urls: BTreeSet<&str> = catalog.entries.iter().flat_map(|e| e.evidence.iter().map(|q| q.url.as_str())).collect();

    // Eight workers share one queue and send their pages back over a channel.
    let queue = Mutex::new(urls.iter().copied().collect::<VecDeque<&str>>());
    let (sender, receiver) = mpsc::channel();
    let agent: ureq::Agent = ureq::Agent::config_builder().timeout_global(Some(Duration::from_secs(180))).build().into();
    thread::scope(|scope| {
        for _ in 0..8 {
            let sender = sender.clone();
            let (queue, agent, cache) = (&queue, &agent, &cache);
            scope.spawn(move || {
                while let Some(url) = queue.lock().unwrap().pop_front() {
                    let _ = sender.send((url, readings(url, cache, token, agent)));
                }
            });
        }
    });
    drop(sender);

    let mut pages: HashMap<&str, Vec<String>> = HashMap::new();
    let mut unreachable = Vec::new();
    for (url, result) in receiver {
        match result {
            Ok(texts) => {
                pages.insert(url, texts);
            }
            Err(error) => unreachable.push(format!("{url}: {error}")),
        }
    }

    let mut failures = Vec::new();
    for entry in &catalog.entries {
        for evidence in &entry.evidence {
            let Some(texts) = pages.get(evidence.url.as_str()) else { continue };
            let quote = normalize(&decode_entities(&evidence.quote));
            if !texts.iter().any(|text| text.contains(&quote)) {
                let shown: String = evidence.quote.chars().take(160).collect();
                failures.push(format!("{}: quote not found at {}\n    \"{shown}\"", entry.slug(), evidence.url));
            }
        }
    }
    unreachable.sort();
    Report { failures, unreachable }
}

/// The normalized readings of a page, from the cache or the network.
fn readings(url: &str, cache: &Path, token: Option<&str>, agent: &ureq::Agent) -> Result<Vec<String>, String> {
    let file: PathBuf = cache.join(format!("{:016x}", fnv1a(url)));
    let body = match fs::read_to_string(&file) {
        Ok(cached) => cached,
        Err(_) => {
            let body = fetch(url, token, agent)?;
            let _ = fs::write(&file, &body);
            body
        }
    };

    let mut texts = vec![body.clone()];
    let lower = body.to_ascii_lowercase();
    if lower.contains("<html") || lower.contains("</div>") {
        let without_code = remove_blocks(&remove_blocks(&body, "script"), "style");
        texts.push(decode_entities(&strip_tags(&without_code, "")));
        texts.push(decode_entities(&strip_tags(&without_code, " ")));
    }
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
        let mut found = Vec::new();
        strings(&json, &mut found);
        texts.push(found.join("\n"));
    }
    Ok(texts.iter().map(|text| normalize(text)).collect())
}

/// GitHub pages are read from their raw source, which is where the text a quote was copied from lives.
fn fetch(address: &str, token: Option<&str>, agent: &ureq::Agent) -> Result<String, String> {
    let parts: Vec<&str> = address.split('/').collect();
    let mut address = address.to_string();
    let mut raw_readme = false;
    if address.starts_with("https://github.com/") && parts.len() > 6 && parts[5] == "blob" {
        // https://github.com/owner/repo/blob/ref/path → https://raw.githubusercontent.com/owner/repo/ref/path
        address = format!("https://raw.githubusercontent.com/{}/{}/{}", parts[3], parts[4], parts[6..].join("/"));
        address.truncate(address.find('#').unwrap_or(address.len()));
    } else if address.starts_with("https://github.com/")
        && (parts.len() <= 5 || parts[5].is_empty() || parts[5].starts_with('#'))
    {
        let repo = parts[4].split('#').next().unwrap_or(parts[4]);
        address = format!("https://api.github.com/repos/{}/{repo}/readme", parts[3]);
        raw_readme = true;
    }

    let mut request = agent.get(&address).header("User-Agent", "fastrust-catalog");
    if raw_readme {
        request = request.header("Accept", "application/vnd.github.raw");
    }
    if let (true, Some(token)) = (address.starts_with("https://api.github.com/"), token) {
        request = request.header("Authorization", &format!("Bearer {token}"));
    }
    let mut response = request.call().map_err(|e| e.to_string())?;
    response.body_mut().with_config().limit(64 * 1024 * 1024).read_to_string().map_err(|e| e.to_string())
}

fn strings(json: &serde_json::Value, found: &mut Vec<String>) {
    match json {
        serde_json::Value::String(text) => found.push(text.clone()),
        serde_json::Value::Array(items) => items.iter().for_each(|item| strings(item, found)),
        serde_json::Value::Object(object) => object.values().for_each(|value| strings(value, found)),
        _ => {}
    }
}

/// Removes every `<tag …>…</tag>` block, ignoring case.
fn remove_blocks(text: &str, tag: &str) -> String {
    // ASCII lowercasing keeps byte offsets lined up with `text`.
    let lower = text.to_ascii_lowercase();
    let (open, close) = (format!("<{tag}"), format!("</{tag}>"));
    let mut result = String::with_capacity(text.len());
    let mut position = 0;
    while let Some(start) = lower[position..].find(&open).map(|i| i + position) {
        result.push_str(&text[position..start]);
        result.push(' ');
        match lower[start..].find(&close) {
            Some(end) => position = start + end + close.len(),
            None => return result,
        }
    }
    result.push_str(&text[position..]);
    result
}

/// Replaces every `<…>` with `separator`.
fn strip_tags(text: &str, separator: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut inside = false;
    for c in text.chars() {
        match (inside, c) {
            (false, '<') => inside = true,
            (true, '>') => {
                inside = false;
                result.push_str(separator);
            }
            (false, c) => result.push(c),
            (true, _) => {}
        }
    }
    result
}

/// Lowercased, typographic quotes and dashes folded to ASCII, whitespace collapsed to single spaces.
pub fn normalize(text: &str) -> String {
    let folded: String = text
        .to_lowercase()
        .chars()
        .map(|c| match c {
            '\u{2018}' | '\u{2019}' => '\'',
            '\u{201C}' | '\u{201D}' => '"',
            '\u{2013}' | '\u{2014}' | '\u{2212}' => '-',
            '\u{00A0}' => ' ',
            c => c,
        })
        .collect();
    folded.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn decode_entities(text: &str) -> String {
    if !text.contains('&') {
        return text.to_string();
    }
    let named = [
        ("&quot;", "\""), ("&#39;", "'"), ("&#x27;", "'"), ("&apos;", "'"), ("&lt;", "<"), ("&gt;", ">"),
        ("&nbsp;", " "), ("&#x2F;", "/"), ("&#47;", "/"),
    ];
    let mut result = text.to_string();
    for (entity, character) in named {
        result = result.replace(entity, character);
    }
    // Numeric references: &#8212; and &#x2014;.
    let mut decoded = String::with_capacity(result.len());
    let mut rest = result.as_str();
    while let Some(start) = rest.find("&#") {
        decoded.push_str(&rest[..start]);
        let after = &rest[start + 2..];
        let (hex, digits) = match after.strip_prefix(['x', 'X']) {
            Some(hex_digits) => (true, hex_digits),
            None => (false, after),
        };
        let end = digits.find(';');
        let value = end.and_then(|end| u32::from_str_radix(&digits[..end], if hex { 16 } else { 10 }).ok());
        match (value.and_then(char::from_u32), end) {
            (Some(c), Some(end)) => {
                decoded.push(c);
                rest = &digits[end + 1..];
            }
            _ => {
                decoded.push_str("&#");
                rest = after;
            }
        }
    }
    decoded.push_str(rest);
    decoded.replace("&amp;", "&")
}

/// A stable name for a cached page. std's hasher is randomly seeded, so it can't name files across runs.
fn fnv1a(text: &str) -> u64 {
    text.bytes().fold(0xcbf2_9ce4_8422_2325, |hash, byte| (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizing() {
        assert_eq!(normalize("It’s  “Fast”\n—\u{00A0}2×"), "it's \"fast\" - 2×");
    }

    #[test]
    fn entities() {
        assert_eq!(decode_entities("a &amp;lt; b &#8212; c &#x2014; d &#39;e&#39;"), "a &lt; b — c — d 'e'");
        assert_eq!(decode_entities("&#bad; &#999999999;"), "&#bad; &#999999999;");
    }

    #[test]
    fn tags() {
        let page = "<p>2.4<b>x</b> faster</p><script>var x = '<p>';</script><STYLE>p{}</STYLE>";
        let without = remove_blocks(&remove_blocks(page, "script"), "style");
        assert_eq!(normalize(&strip_tags(&without, "")), "2.4x faster");
        assert_eq!(normalize(&strip_tags(&without, " ")), "2.4 x faster");
    }
}
