//! Validates that `test_documents/html/MANIFEST.toml` and the files on disk under
//! `test_documents/html/` agree with each other: every manifest entry points at a real
//! file, and every HTML file added under a manifest-tracked family directory is listed
//! in the manifest.
//!
//! ~keep This is a data-integrity check, not a conversion oracle. Panic-safety for every
//! ~keep file added here is already covered unconditionally by `corpus_robustness.rs`'s
//! ~keep own recursive walk of `test_documents/html/` (no wiring needed), and Tier-1/
//! ~keep Tier-2 byte-parity is a separate, concurrently-developed harness over the
//! ~keep existing corpora. This test's only job is to keep the manifest from silently
//! ~keep drifting from the filesystem as fixtures are added or removed.

#![allow(missing_docs)]
#![allow(clippy::print_stdout, clippy::print_stderr, clippy::dbg_macro)] // ~keep: test crate

mod support;

use std::collections::BTreeSet;

use support::{collect_html_files, corpus_root, load_manifest};

/// Family directories tracked by `MANIFEST.toml`.
///
/// ~keep The pre-existing `wikipedia/`, `issues/`, and `visitor/` directories predate the
/// ~keep manifest and are intentionally out of scope here -- backfilling manifest coverage
/// ~keep for them is a separate task, not something this corpus-expansion work should do
/// ~keep incidentally.
const MANIFEST_TRACKED_FAMILIES: &[&str] = &[
    "discourse",
    "docgen-docusaurus",
    "docgen-doxygen",
    "docgen-javadoc",
    "docgen-mkdocs",
    "docgen-rustdoc",
    "docgen-sphinx",
    "drupal",
    "email",
    "ghost",
    "github-markdown",
    "govt",
    "gutenberg",
    "html5lib",
    "legacy",
    "mdn",
    "news-ecommerce",
    "office-gdocs",
    "office-libreoffice",
    "office-word",
    "squarespace",
    "stackoverflow",
    "wordpress",
];

#[test]
fn manifest_matches_filesystem_exactly() {
    let root = corpus_root();
    let documents = load_manifest();
    assert!(
        !documents.is_empty(),
        "MANIFEST.toml at {} resolved to zero entries -- the loader would be hollow",
        root.join("MANIFEST.toml").display()
    );

    let manifest_paths: BTreeSet<String> = documents.iter().map(|d| d.path.clone()).collect();

    let mut actual_paths = BTreeSet::new();
    for family in MANIFEST_TRACKED_FAMILIES {
        let dir = root.join(family);
        let mut files = Vec::new();
        collect_html_files(&dir, &mut files);
        for file in files {
            let rel = file
                .strip_prefix(&root)
                .expect("family dir is under corpus root")
                .to_string_lossy()
                .replace('\\', "/");
            actual_paths.insert(rel);
        }
    }

    let missing_on_disk: Vec<&String> = manifest_paths.difference(&actual_paths).collect();
    let missing_in_manifest: Vec<&String> = actual_paths.difference(&manifest_paths).collect();

    assert!(
        missing_on_disk.is_empty(),
        "MANIFEST.toml lists file(s) not found on disk: {missing_on_disk:?}"
    );
    assert!(
        missing_in_manifest.is_empty(),
        "found HTML file(s) under a manifest-tracked family with no MANIFEST.toml entry: {missing_in_manifest:?}"
    );
}

#[test]
fn every_document_is_readable_and_non_empty() {
    for doc in load_manifest() {
        let html = doc.html();
        assert!(!html.trim().is_empty(), "{} ({}) is empty", doc.path, doc.family);
    }
}

#[test]
fn every_document_has_required_manifest_fields_populated() {
    for doc in load_manifest() {
        assert!(!doc.family.is_empty(), "{}: family is empty", doc.path);
        assert!(!doc.source.is_empty(), "{}: source is empty", doc.path);
        assert!(!doc.license.is_empty(), "{}: license is empty", doc.path);
        assert!(!doc.retrieved.is_empty(), "{}: retrieved is empty", doc.path);
        assert!(!doc.features.is_empty(), "{}: features is empty", doc.path);
        if doc.synthesized {
            assert_eq!(
                doc.source, "synthesized",
                "{}: synthesized=true but source is {:?}, expected \"synthesized\"",
                doc.path, doc.source
            );
        } else {
            assert_ne!(
                doc.source, "synthesized",
                "{}: synthesized=false but source is literally \"synthesized\"",
                doc.path
            );
        }
    }
}
