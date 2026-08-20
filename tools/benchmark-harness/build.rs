//! Embeds build-time profile and compiler flags in benchmark provenance.

fn main() {
    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "unverified".to_owned());
    let rustflags = std::env::var("CARGO_ENCODED_RUSTFLAGS").unwrap_or_default();
    println!("cargo:rustc-env=HTMBENCH_COMPILED_PROFILE={profile}");
    println!("cargo:rustc-env=HTMBENCH_COMPILED_RUSTFLAGS={rustflags}");
    println!("cargo:rerun-if-env-changed=CARGO_ENCODED_RUSTFLAGS");
}
