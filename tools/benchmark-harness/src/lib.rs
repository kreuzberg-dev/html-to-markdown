//! Benchmark harness library for html-to-markdown-rs.
//!
//! Provides fixture loading, timing, snapshot comparison, and survey utilities
//! consumed by the `htmbench` binary.

#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions
)]

pub mod bench;
pub mod calibration;
pub mod fixture;
pub mod oracle;
pub mod policy;
pub mod provenance;
pub mod schema;
pub mod stats;
pub mod survey;
