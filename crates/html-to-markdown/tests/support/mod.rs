//! Shared test support: a loader/enumeration helper for the real-world HTML corpus at
//! `test_documents/html/` (see `MANIFEST.toml` there for full per-document provenance).
//!
//! ~keep Not a `tests/*.rs` file itself -- nested under `tests/support/` so Cargo does not
//! ~keep compile it as its own integration-test binary. A consuming test file declares
//! ~keep `mod support;` and Cargo/rustc resolve that to this file relative to the test
//! ~keep file's own directory, the standard idiom for sharing code between integration
//! ~keep tests without a separate crate.

#![allow(dead_code)] // ~keep: not every consuming test file uses every helper here.
#![allow(missing_docs)] // ~keep: test-support module, not part of the public crate API.

use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

/// One row of `test_documents/html/MANIFEST.toml`.
#[derive(Debug, Clone, Deserialize)]
pub struct CorpusDocument {
    /// Path relative to `test_documents/html/`.
    pub path: String,
    /// Generator/producer family (one subdirectory per family).
    pub family: String,
    /// Origin URL, or `"synthesized"` for a hand-built fixture.
    pub source: String,
    /// Governing license, or an "N/A (original work)" note for synthesized fixtures.
    pub license: String,
    /// ISO 8601 capture/authoring date.
    pub retrieved: String,
    /// `true` for hand-built fixtures mimicking a generator's structural signature,
    /// `false` for vendored/real content.
    pub synthesized: bool,
    /// One-line summary of the structural constructs this fixture exercises.
    pub features: String,
}

#[derive(Debug, Deserialize)]
struct ManifestFile {
    document: Vec<CorpusDocument>,
}

/// Absolute path to `test_documents/html/`, resolved from `CARGO_MANIFEST_DIR`.
///
/// ~keep Mirrors `corpus_robustness.rs`'s `optional_extra_roots` path
/// ~keep (`../../../test_documents/html`) -- `test_documents` lives at the workspace
/// ~keep root, two levels above `crates/html-to-markdown`.
pub fn corpus_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../test_documents/html")
}

/// Loads and parses `MANIFEST.toml`.
///
/// # Panics
///
/// Panics if the manifest is missing or fails to parse -- a loader that silently
/// returned an empty corpus on a malformed manifest would defeat the point of
/// tracking one.
pub fn load_manifest() -> Vec<CorpusDocument> {
    let manifest_path = corpus_root().join("MANIFEST.toml");
    let raw = fs::read_to_string(&manifest_path).unwrap_or_else(|e| panic!("reading {}: {e}", manifest_path.display()));
    let parsed: ManifestFile =
        toml::from_str(&raw).unwrap_or_else(|e| panic!("parsing {}: {e}", manifest_path.display()));
    parsed.document
}

impl CorpusDocument {
    /// Absolute path to this document's HTML file.
    pub fn abs_path(&self) -> PathBuf {
        corpus_root().join(&self.path)
    }

    /// Reads the document's HTML content.
    ///
    /// # Panics
    ///
    /// Panics if the file listed in the manifest does not exist on disk.
    pub fn html(&self) -> String {
        let path = self.abs_path();
        fs::read_to_string(&path).unwrap_or_else(|e| panic!("reading {}: {e}", path.display()))
    }
}

/// Loads every manifest entry belonging to one `family`.
pub fn documents_in_family(family: &str) -> Vec<CorpusDocument> {
    load_manifest().into_iter().filter(|d| d.family == family).collect()
}

/// Every distinct family name recorded in the manifest, sorted and de-duplicated.
pub fn families() -> Vec<String> {
    let mut names: Vec<String> = load_manifest().into_iter().map(|d| d.family).collect();
    names.sort();
    names.dedup();
    names
}

/// Recursively collects every `*.html` file under `dir`.
///
/// ~keep A near-identical `collect_html` already lives inline in `corpus_robustness.rs`.
/// ~keep Not consolidated onto this shared copy here because that file is owned by a
/// ~keep different, concurrently-running task in this session; duplicating ~10 lines is
/// ~keep cheaper than a cross-task merge conflict on a file this task must not edit.
pub fn collect_html_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_html_files(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "html") {
            out.push(path);
        }
    }
}
