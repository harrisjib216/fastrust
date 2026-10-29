//! The fastrust catalog tool: checks the records in data/, verifies their quotes against the pages they came from,
//! and renders catalog/ and the README's generated sections.

mod check;
mod model;
mod render;
mod verify;

use std::path::Path;
use std::process::ExitCode;

const USAGE: &str = "usage: cargo run -p catalog -- <command>

  check    validate every record in data/entries against the schema
  verify   fetch every evidence URL and check every quote is on the page
  render   write catalog/, the README's generated sections, and the JSONL and CSV exports";

fn main() -> ExitCode {
    // The repository root: the folder above tools/.
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("tools/ has a parent");
    let catalog = match model::Catalog::load(root) {
        Ok(catalog) => catalog,
        Err(error) => {
            eprintln!("catalog: {error}");
            return ExitCode::FAILURE;
        }
    };

    match std::env::args().nth(1).as_deref() {
        Some("check") => {
            let problems = check::check(&catalog);
            for problem in &problems {
                println!("{problem}");
            }
            println!("{} entries, {} problems", catalog.entries.len(), problems.len());
            if problems.is_empty() { ExitCode::SUCCESS } else { ExitCode::FAILURE }
        }
        Some("verify") => {
            let token = std::env::var("GITHUB_TOKEN").ok();
            let report = verify::verify(&catalog, token.as_deref());
            for line in &report.unreachable {
                println!("unreachable: {line}");
            }
            for line in &report.failures {
                println!("{line}");
            }
            let quotes: usize = catalog.entries.iter().map(|e| e.evidence.len()).sum();
            println!(
                "{quotes} quotes, {} not found, {} pages unreachable",
                report.failures.len(),
                report.unreachable.len()
            );
            if report.failures.is_empty() && report.unreachable.is_empty() { ExitCode::SUCCESS } else { ExitCode::FAILURE }
        }
        Some("render") => {
            let problems = check::check(&catalog);
            if !problems.is_empty() {
                eprintln!("{}\nfix these before rendering", problems.join("\n"));
                return ExitCode::FAILURE;
            }
            match render::render(&catalog) {
                Ok(()) => {
                    println!("rendered {} entries", catalog.entries.len());
                    ExitCode::SUCCESS
                }
                Err(error) => {
                    eprintln!("catalog: {error}");
                    ExitCode::FAILURE
                }
            }
        }
        _ => {
            eprintln!("{USAGE}");
            ExitCode::FAILURE
        }
    }
}
