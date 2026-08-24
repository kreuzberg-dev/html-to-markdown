#!/usr/bin/env python3
"""
Vendor html-to-markdown-rs core crate into R package.

This script:
1. Reads workspace.dependencies and version from root Cargo.toml
2. Copies crates/html-to-markdown/ to packages/r/src/rust/vendor/html-to-markdown-rs/
3. Replaces workspace = true with explicit values in the vendored Cargo.toml,
   including materializing the root [workspace.lints] sub-tree as the vendored
   crate's own [lints] table
"""

import os
import re
import shutil
import sys
from pathlib import Path

try:
    import tomllib
except ImportError:
    import tomli as tomllib  # type: ignore


# ~keep Matches `[workspace.lints]` and every `[workspace.lints.<tool>]` header. Only
# headers at column 0 are table headers in TOML.
WORKSPACE_LINTS_HEADER_RE = re.compile(r"^\[workspace\.lints(\.[^\]]+)?\]\s*$")
# ~keep Any table header at column 0 ends the `[workspace.lints]` run.
TABLE_HEADER_RE = re.compile(r"^\[")
# ~keep The inheritance marker a workspace member uses to pull in `[workspace.lints]`.
CRATE_LINTS_INHERIT_RE = re.compile(r"(?m)^\[lints\]\nworkspace = true\n?")


def get_repo_root() -> Path:
    """Get repository root directory."""
    repo_root_env = os.environ.get("REPO_ROOT")
    if repo_root_env:
        return Path(repo_root_env)

    script_dir = Path(__file__).parent.absolute()
    return (script_dir / ".." / ".." / "..").resolve()


def read_toml(path: Path) -> dict[str, object]:
    """Read a TOML file and return its contents."""
    with path.open("rb") as f:
        return tomllib.load(f)


def get_workspace_config(repo_root: Path) -> tuple[str, dict[str, object], dict[str, object], dict[str, object]]:
    """Extract version, package metadata, dependencies, and lints from root Cargo.toml."""
    data = read_toml(repo_root / "Cargo.toml")
    ws = data.get("workspace", {})
    version = ws.get("package", {}).get("version", "0.0.0")
    pkg = ws.get("package", {})
    deps = ws.get("dependencies", {})
    lints = ws.get("lints", {})
    return version, pkg, deps, lints


def render_workspace_lints(repo_root: Path, ws_lints: dict[str, object]) -> str:
    """Render the root manifest's ``[workspace.lints]`` sub-tree as a crate-level
    ``[lints]`` sub-tree, verbatim.

    A crate lifted out of its workspace must keep every piece of build configuration it
    was inheriting, not just the parts that happen to be load-bearing today. ``[lints]``
    is resolved through the workspace, so vendoring has to materialize it -- otherwise
    the vendored copy compiles under a *different* lint configuration than the sources
    it was copied from. The entry that matters most is ``unexpected_cfgs``' check-cfg
    allowlist: it is what declares the crate's own ``#[cfg(...)]`` gates as expected cfg
    names. Drop it and every gate becomes an ``unexpected_cfgs`` diagnostic -- silent in
    a default build, a hard error under ``RUSTFLAGS="-D warnings"``.

    Nothing here is specific to any one lint or cfg name: the whole sub-tree is copied,
    so a workspace that adds a lint or a check-cfg entry needs no change here. The
    extracted text is re-parsed and compared against the authoritative parse of the root
    manifest, so a mis-extraction fails loudly instead of silently emitting a different
    lint configuration -- which is the failure this function exists to prevent. ~keep
    """
    if not ws_lints:
        return ""

    block: list[str] = []
    capturing = False
    for line in (repo_root / "Cargo.toml").read_text(encoding="utf-8").splitlines():
        if WORKSPACE_LINTS_HEADER_RE.match(line):
            capturing = True
            block.append("[lints" + line.strip()[len("[workspace.lints") :])
            continue
        if capturing and TABLE_HEADER_RE.match(line):
            break
        if capturing:
            block.append(line)

    rendered = "\n".join(block).rstrip() + "\n"
    if tomllib.loads(rendered).get("lints") != ws_lints:
        raise RuntimeError(
            "extracted [workspace.lints] does not round-trip to the parsed workspace lints; "
            "refusing to vendor a crate under a lint configuration that differs from its source"
        )
    return rendered


def format_dependency(name: str, dep_spec: object) -> str:
    """Format a dependency spec for Cargo.toml."""
    if isinstance(dep_spec, str):
        return f'{name} = "{dep_spec}"'
    if isinstance(dep_spec, dict):
        parts: list[str] = []

        package = dep_spec.get("package")
        if package:
            parts.append(f'package = "{package}"')

        version = dep_spec.get("version", "")
        parts.append(f'version = "{version}"')

        features = dep_spec.get("features", [])
        if features:
            features_str = ", ".join(f'"{f}"' for f in features)
            parts.append(f"features = [{features_str}]")

        default_features = dep_spec.get("default-features")
        if default_features is False:
            parts.append("default-features = false")

        spec_str = ", ".join(parts)
        return f"{name} = {{ {spec_str} }}"

    return f'{name} = "{dep_spec}"'


def _replace_package_fields(content: str, version: str, pkg: dict[str, object], lints_block: str) -> str:
    """Replace package-level workspace inheritance fields."""
    content = re.sub(r"^version\.workspace = true$", f'version = "{version}"', content, flags=re.MULTILINE)
    content = re.sub(
        r"^edition\.workspace = true$", f'edition = "{pkg.get("edition", "2024")}"', content, flags=re.MULTILINE
    )
    content = re.sub(
        r"^rust-version\.workspace = true$",
        f'rust-version = "{pkg.get("rust-version", "1.85")}"',
        content,
        flags=re.MULTILINE,
    )

    authors = pkg.get("authors", [])
    if authors:
        authors_str = ", ".join(f'"{a}"' for a in authors)
        content = re.sub(r"^authors\.workspace = true$", f"authors = [{authors_str}]", content, flags=re.MULTILINE)

    for field in ("license", "repository", "homepage", "documentation"):
        default = "MIT" if field == "license" else ""
        content = re.sub(
            rf"^{field}\.workspace = true$",
            f'{field} = "{pkg.get(field, default)}"',
            content,
            flags=re.MULTILINE,
        )

    # Replace the workspace inheritance marker with the workspace's own lint tables.
    # When the workspace declares none there is nothing to inline, and `workspace = true`
    # with no parent workspace is not a manifest cargo can read -- so strip it. ~keep
    return CRATE_LINTS_INHERIT_RE.sub(lambda _match: lints_block, content, count=1)


def _make_fields_replacer(dep_name: str, dep_spec: object) -> callable:
    """Create a regex replacer that merges workspace dep spec with extra fields."""

    def replacer(match: re.Match[str]) -> str:
        other_fields_str = match.group(1).strip()
        base_spec = format_dependency(dep_name, dep_spec)

        if " = { " not in base_spec:
            version_val = base_spec.split(" = ", 1)[1].strip('"')
            spec_part = f'version = "{version_val}"'
        else:
            spec_part = base_spec.split(" = { ", 1)[1].rstrip("}")

        existing_keys: set[str] = set()
        for raw_part in spec_part.split(","):
            stripped = raw_part.strip()
            if "=" in stripped:
                existing_keys.add(stripped.split("=")[0].strip())

        filtered_fields: list[str] = []
        for raw_field in other_fields_str.split(","):
            stripped = raw_field.strip()
            if stripped and "=" in stripped:
                if stripped.split("=")[0].strip() not in existing_keys:
                    filtered_fields.append(stripped)
            elif stripped:
                filtered_fields.append(stripped)

        if filtered_fields:
            return f"{dep_name} = {{ {spec_part}, {', '.join(filtered_fields)} }}"
        return f"{dep_name} = {{ {spec_part} }}"

    return replacer


def replace_workspace_refs(
    toml_path: Path,
    version: str,
    pkg: dict[str, object],
    deps: dict[str, object],
    lints_block: str,
) -> None:
    """Replace workspace references with explicit values in vendored Cargo.toml."""
    with toml_path.open() as f:
        content = f.read()

    content = _replace_package_fields(content, version, pkg, lints_block)

    for name, dep_spec in deps.items():
        pattern_dotted = rf"^{re.escape(name)}\.workspace = true$"
        content = re.sub(pattern_dotted, format_dependency(name, dep_spec), content, flags=re.MULTILINE)

        pattern_simple = rf"^{re.escape(name)} = \{{ workspace = true \}}$"
        content = re.sub(pattern_simple, format_dependency(name, dep_spec), content, flags=re.MULTILINE)

        pattern_extra = rf"^{re.escape(name)} = \{{ workspace = true, (.+?) \}}$"
        content = re.sub(pattern_extra, _make_fields_replacer(name, dep_spec), content, flags=re.MULTILINE | re.DOTALL)

    with toml_path.open("w") as f:
        f.write(content)


def main() -> None:
    """Vendor the html-to-markdown-rs core crate into the R package."""
    repo_root = get_repo_root()
    src_crate = repo_root / "crates" / "html-to-markdown"
    dest_vendor = repo_root / "packages" / "r" / "src" / "rust" / "vendor" / "html-to-markdown-rs"

    print("=== Vendoring html-to-markdown-rs core crate ===")

    if not src_crate.exists():
        print(f"Error: Source crate not found at {src_crate}", file=sys.stderr)
        sys.exit(1)

    version, pkg, deps, ws_lints = get_workspace_config(repo_root)
    lints_block = render_workspace_lints(repo_root, ws_lints)
    print(f"Workspace version: {version}")

    if dest_vendor.exists():
        shutil.rmtree(dest_vendor)
        print("Cleaned existing vendor directory")

    shutil.copytree(src_crate, dest_vendor)
    print("Copied crates/html-to-markdown/ -> vendor/html-to-markdown-rs/")

    for artifact_dir in ["target", ".fastembed_cache"]:
        artifact = dest_vendor / artifact_dir
        if artifact.exists():
            shutil.rmtree(artifact)

    for pattern in ["*.swp", "*.bak", "*.tmp", "*~"]:
        for f in dest_vendor.rglob(pattern):
            f.unlink()

    vendor_toml = dest_vendor / "Cargo.toml"
    if vendor_toml.exists():
        replace_workspace_refs(vendor_toml, version, pkg, deps, lints_block)
        inlined = "inlined [workspace.lints]" if lints_block else "no [workspace.lints] to inline"
        print(f"Updated vendor/html-to-markdown-rs/Cargo.toml ({inlined})")

    print(f"\nVendoring complete (version: {version})")


if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)
