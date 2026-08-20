//! Structured benchmark provenance collection and compatibility checks.

use std::process::Command;

use anyhow::{Context, Result};

use crate::schema::Provenance;

/// Run settings that affect benchmark compatibility.
pub struct CaptureSettings {
    /// Conversion tier selection (`auto`, `tier1`, or `tier2`).
    pub tier_strategy: &'static str,
    /// Visitor selection (`disabled` or `noop`).
    pub visitor_mode: &'static str,
    /// Fixed iterations, or `None` for automatic calibration.
    pub iteration_override: Option<u32>,
}

/// Collect the hardware, toolchain, build, and runner match key.
pub fn collect(settings: &CaptureSettings) -> Result<Provenance> {
    let rustc_verbose = command_output("rustc", &["-Vv"])?;
    let rustc_host = rustc_verbose
        .lines()
        .find_map(|line| line.strip_prefix("host: "))
        .context("rustc -Vv did not report a host")?
        .to_owned();
    Ok(Provenance {
        os: std::env::consts::OS.to_owned(),
        arch: std::env::consts::ARCH.to_owned(),
        cpu_model: cpu_model(),
        cpu_count: std::thread::available_parallelism().map_or(1, std::num::NonZero::get),
        rustc_verbose,
        rustc_host,
        cargo_version: command_output("cargo", &["-V"])?,
        profile: build_profile(),
        build_flags: build_flags(),
        measurement_mode: "nine-batch-median-mad-v2".to_owned(),
        tier_strategy: settings.tier_strategy.to_owned(),
        visitor_mode: settings.visitor_mode.to_owned(),
        iteration_override: settings.iteration_override,
        warmup_iterations: crate::bench::WARMUP_ITERATIONS,
        calibration_target_ms: crate::bench::CALIBRATION_TARGET_MS,
        calibration_timeout_ms: crate::bench::CALIBRATION_TIMEOUT_MS,
        core_features: compiled_features(),
        runner_image: std::env::var("ImageOS").ok(),
        runner_class: std::env::var("RUNNER_ENVIRONMENT")
            .or_else(|_| std::env::var("HTMBENCH_RUNNER_CLASS"))
            .ok(),
    })
}

fn compiled_features() -> Vec<String> {
    let mut features = Vec::new();
    for (enabled, feature) in [
        (cfg!(feature = "inline-images"), "inline-images"),
        (cfg!(feature = "metadata"), "metadata"),
        (cfg!(feature = "serde"), "serde"),
        (cfg!(feature = "visitor"), "visitor"),
    ] {
        if enabled {
            features.push(feature.to_owned());
        }
    }
    if cfg!(feature = "testkit") {
        features.push("testkit".to_owned());
    }
    features.sort();
    features
}

fn build_flags() -> String {
    env!("HTMBENCH_COMPILED_RUSTFLAGS").replace('\u{1f}', " ")
}

fn build_profile() -> String {
    env!("HTMBENCH_COMPILED_PROFILE").to_owned()
}

fn command_output(program: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .with_context(|| format!("running {program}"))?;
    anyhow::ensure!(output.status.success(), "{program} exited with {}", output.status);
    String::from_utf8(output.stdout)
        .with_context(|| format!("decoding {program} output"))
        .map(|value| value.trim().to_owned())
}

fn cpu_model() -> String {
    if let Ok(cpu_info) = std::fs::read_to_string("/proc/cpuinfo")
        && let Some(model) = cpu_info.lines().find_map(|line| line.strip_prefix("model name\t: "))
    {
        return model.to_owned();
    }
    command_output("sysctl", &["-n", "machdep.cpu.brand_string"]).unwrap_or_else(|_| "unknown".to_owned())
}
