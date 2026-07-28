#!/usr/bin/env python3
"""Sync the coding-agent plugin version to the core ``Cargo.toml`` version.

``ai-rulez generate --plugin`` renders every per-runtime plugin manifest under
``plugin/`` from ``plugin/.ai-rulez/config.toml``'s ``[plugin].version``. This script
keeps that single source pinned to the crate version in ``Cargo.toml`` (the repo's
version source of truth), the same way ``alef sync-versions`` pins the alef-managed
binding manifests. Run via ``task version:sync`` (which then regenerates the bundles);
freshness of the generated bundles themselves is enforced by ``ai-rulez verify
--plugin`` in CI, so this script only touches the config source.

Usage:
    python3 scripts/sync_plugin_version.py           # apply
    python3 scripts/sync_plugin_version.py --check    # verify (CI), exit 1 on drift
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PLUGIN_CONFIG = ROOT / "plugin" / ".ai-rulez" / "config.toml"


def core_version() -> str:
    """Read the workspace crate version from ``Cargo.toml``."""
    text = (ROOT / "Cargo.toml").read_text(encoding="utf-8")
    match = re.search(r'(?m)^version = "([^"]+)"', text)
    if not match:
        sys.exit("could not read version from Cargo.toml")
    return match.group(1)


def transform(version: str) -> tuple[str, str]:
    """Rewrite ``version`` inside the ``[plugin]`` table (not the top-level ai-rulez
    schema ``version = "4.0"``) of the plugin config. Uses the native/semver form,
    which is also what ai-rulez expects; ai-rulez renders the PEP 440 form for the
    Hermes wheel.
    """
    original = PLUGIN_CONFIG.read_text(encoding="utf-8")
    updated = re.sub(
        r'(\[plugin\][^\[]*?\nversion = ")[^"]*(")',
        rf"\g<1>{version}\g<2>",
        original,
        count=1,
        flags=re.DOTALL,
    )
    return original, updated


def main() -> int:
    check = "--check" in sys.argv[1:]
    version = core_version()
    original, updated = transform(version)
    if original == updated:
        return 0
    if check:
        print(f"plugin version out of sync with core {version}: {PLUGIN_CONFIG.relative_to(ROOT)}")
        return 1
    PLUGIN_CONFIG.write_text(updated, encoding="utf-8")
    print(f"synced plugin version -> {version}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
