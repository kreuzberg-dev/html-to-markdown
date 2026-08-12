#!/usr/bin/env python3
"""Prove that every generated language binding actually parses.

The generator's own gates cannot catch a template that emits unparseable code:
snapshot tests compare against blessed strings (which are regenerated alongside
the template) and ``alef verify`` re-hashes the files it just wrote, so both are
green by construction. This script is an *external* instrument -- it hands the
generated source to each language's own front end and asks whether it parses.

Two rules keep the result honest:

1. **Parse-only, never compile.** A missing Jackson jar or an unresolved
   ``import vitest`` is not a syntax error, and a checker that reports one would
   produce false failures that train readers to ignore this tool. Every checker
   here stops at the parse phase (``gofmt -e``, ``zig ast-check``,
   ``swiftc -parse``, ``JavacTask.parse()``, ``Code.string_to_quoted!``, ...).
   Where a language ships no parse-only front end, the language is SKIPPED with
   that reason rather than run through a full compile.
2. **A skip is never a pass.** Every language reports one of PASS, FAIL,
   PARTIAL, SKIPPED or NO SOURCES, and the summary prints the counts. A sweep
   that silently skips half its surface while printing green is worse than no
   sweep at all.

``--self-test`` is the guard against this script becoming green-by-construction
in its own right: it feeds each checker a deliberately broken sample and asserts
the checker rejects it. A checker that cannot fail is not proving anything.
``node --check`` on a ``.ts`` file was dropped for exactly this reason -- it
exits 0 on syntactically invalid TypeScript.

The file set comes from ``git ls-files``: ``packages/`` carries build caches
(4000+ vendored ``.rs`` under ``packages/r``, a Zig stdlib cache under
``packages/zig``) that a filesystem walk would sweep up as "generated bindings".

Usage:
    python3 scripts/check_binding_parse.py              # sweep the tree
    python3 scripts/check_binding_parse.py --strict     # skips are fatal too
    python3 scripts/check_binding_parse.py --self-test  # prove the checkers bite
    python3 scripts/check_binding_parse.py --with-cargo # include cargo check
    python3 scripts/check_binding_parse.py --json       # machine-readable
"""

from __future__ import annotations

import argparse
import json
import re
import shutil
import subprocess
import tempfile
from dataclasses import dataclass, field
from functools import cache
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
ALEF_CONFIG = ROOT / "alef.toml"

EXIT_OK = 0
EXIT_FAILED = 1
EXIT_SKIPPED_STRICT = 2

# Per-checker wall-clock ceiling. A front end that hangs (waiting on a package
# fetch, a daemon handshake) must be reported as a failure, not left to stall a
# CI job until the job timeout blames an unrelated step. ~keep
CHECK_TIMEOUT_SECONDS = 300

# TypeScript numbers *syntactic* diagnostics in the 1000-1999 band and semantic
# ones from 2000 up. That split is what lets tsc serve as a parse-only checker:
# a generated test importing an uninstalled `vitest` reports TS2307 (ignored),
# while a malformed parameter list reports TS1138 (fatal). Verified against a
# known-bad sample in --self-test. ~keep
TS_SYNTACTIC_MAX_CODE = 2000
TS_DIAGNOSTIC_RE = re.compile(r"error TS(\d+):")

# javac has no parse-only command-line mode, but `JavacTask.parse()` is exactly
# that: it runs the parser and stops before symbol resolution. Run via the
# single-file source launcher so no build step and no checked-in helper is
# needed. ~keep
JAVA_PARSE_HELPER = """
import com.sun.source.util.JavacTask;
import java.io.File;
import java.util.ArrayList;
import java.util.List;
import javax.tools.Diagnostic;
import javax.tools.DiagnosticCollector;
import javax.tools.JavaCompiler;
import javax.tools.JavaFileObject;
import javax.tools.StandardJavaFileManager;
import javax.tools.ToolProvider;

public class ParseOnly {
    public static void main(String[] args) throws Exception {
        JavaCompiler compiler = ToolProvider.getSystemJavaCompiler();
        DiagnosticCollector<JavaFileObject> diagnostics = new DiagnosticCollector<>();
        StandardJavaFileManager fileManager = compiler.getStandardFileManager(diagnostics, null, null);
        List<File> files = new ArrayList<>();
        for (String argument : args) {
            files.add(new File(argument));
        }
        JavacTask task = (JavacTask) compiler.getTask(
                null, fileManager, diagnostics, null, null, fileManager.getJavaFileObjectsFromFiles(files));
        task.parse();
        int errors = 0;
        for (Diagnostic<? extends JavaFileObject> diagnostic : diagnostics.getDiagnostics()) {
            if (diagnostic.getKind() == Diagnostic.Kind.ERROR) {
                errors++;
                System.out.println(
                        diagnostic.getSource().getName() + ":" + diagnostic.getLineNumber() + ": "
                                + diagnostic.getMessage(null));
            }
        }
        System.exit(errors == 0 ? 0 : 1);
    }
}
"""


@dataclass(frozen=True)
class Checker:
    """One language front end, invoked in parse-only mode."""

    tool: str
    how: str
    suffixes: tuple[str, ...]
    arguments: tuple[str, ...] = ()
    # A sample that MUST be rejected, proving the checker actually bites.
    bad_sample: str = ""
    bad_suffix: str = ""
    # tsc reports semantic errors even under --noResolve; only 1xxx codes count.
    syntactic_codes_only: bool = False
    # The JDK helper is materialised at run time and prepended to the argv.
    needs_java_helper: bool = False
    # `swiftc -parse` refuses two files sharing a basename (e2e/swift and
    # e2e/swift_e2e both ship a Package.swift), so it runs one file per call. ~keep
    one_file_at_a_time: bool = False
    # `gofmt -l` and `zig fmt --check` exit nonzero for merely *unformatted*
    # (but perfectly parseable) files, naming them on stdout, while a genuine
    # parse error goes to stderr. Formatting is poly's job, not this sweep's, so
    # for these two only stderr decides. ~keep
    failure_from_stderr: bool = False

    def command(self, files: list[Path], helper: Path | None) -> list[str]:
        """Build the argv that runs this checker over ``files``."""
        prefix = [str(helper)] if self.needs_java_helper and helper is not None else []
        return [self.tool, *self.arguments, *prefix, *(str(path) for path in files)]


@dataclass(frozen=True)
class Language:
    """One binding surface: where its generated sources live and how to parse them."""

    name: str
    roots: tuple[str, ...]
    checkers: tuple[Checker, ...] = ()
    # Set when no parse-only front end exists for this language at all.
    no_parse_only_reason: str = ""
    # Set when the only available check is a full cargo build.
    requires_cargo: bool = False
    # Source extensions this language ships that no checker here can parse.
    # These count against the denominator; READMEs, licences and lockfiles under
    # the same roots are not source and must not dilute the coverage figure. ~keep
    unchecked_suffixes: tuple[str, ...] = ()

    @property
    def source_suffixes(self) -> tuple[str, ...]:
        """File extensions this checker claims."""
        checked = {suffix for checker in self.checkers for suffix in checker.suffixes}
        return tuple(sorted(checked | set(self.unchecked_suffixes)))


PYTHON = Checker(
    tool="python3",
    how="compile() builtin (parse + bytecode, no .pyc written)",
    suffixes=(".py", ".pyi"),
    arguments=("-c", "import sys\nfor f in sys.argv[1:]: compile(open(f).read(), f, 'exec')"),
    bad_sample="def f(:\n",
    bad_suffix=".py",
)

RUBY = Checker(
    tool="ruby",
    how="ruby -c",
    suffixes=(".rb",),
    arguments=("-c",),
    bad_sample="def f(\nend\n",
    bad_suffix=".rb",
)

PHP = Checker(
    tool="php",
    how="php -l",
    suffixes=(".php",),
    arguments=("-l",),
    bad_sample="<?php function f( { }\n",
    bad_suffix=".php",
)

GO = Checker(
    tool="gofmt",
    how="gofmt -e (parse only, no build, no type check)",
    suffixes=(".go",),
    arguments=("-e", "-l"),
    bad_sample="package x\nfunc F( {\n",
    bad_suffix=".go",
    failure_from_stderr=True,
)

# `zig ast-check` is deliberately not used: it runs early semantic analysis on
# top of the parse (it rejects unused locals, for instance), so it would report
# "does not parse" for code that parses fine. `zig fmt --check` stops at the
# syntax tree. ~keep
ZIG = Checker(
    tool="zig",
    how="zig fmt --check (parse only; formatting drift ignored)",
    suffixes=(".zig",),
    arguments=("fmt", "--check"),
    bad_sample="pub fn f( {\n",
    bad_suffix=".zig",
    failure_from_stderr=True,
)

SWIFT = Checker(
    tool="swiftc",
    how="swiftc -parse",
    suffixes=(".swift",),
    arguments=("-parse",),
    bad_sample="func f( {\n",
    bad_suffix=".swift",
    one_file_at_a_time=True,
)

JAVA = Checker(
    tool="java",
    how="JavacTask.parse() via the single-file source launcher",
    suffixes=(".java",),
    needs_java_helper=True,
    bad_sample="class B { void f( { }\n",
    bad_suffix=".java",
)

DART = Checker(
    tool="dart",
    how="dart format --output=none (exits 65 only on a parse error)",
    suffixes=(".dart",),
    arguments=("format", "--output=none"),
    bad_sample="void f( {\n",
    bad_suffix=".dart",
)

ELIXIR = Checker(
    tool="elixir",
    how="Code.string_to_quoted! (parse to AST, no compile)",
    suffixes=(".ex", ".exs"),
    arguments=("-e", "for f <- System.argv(), do: Code.string_to_quoted!(File.read!(f), file: f)"),
    bad_sample="defmodule B do\n  def f( do\nend\n",
    bad_suffix=".ex",
)

R = Checker(
    tool="Rscript",
    how="parse() (R's own parser, no evaluation)",
    suffixes=(".R",),
    arguments=("-e", "invisible(lapply(commandArgs(TRUE), parse))"),
    bad_sample="f <- function( {\n",
    bad_suffix=".R",
)

C = Checker(
    tool="cc",
    how="cc -fsyntax-only",
    suffixes=(".c", ".h"),
    arguments=("-fsyntax-only", f"-I{ROOT / 'crates' / 'html-to-markdown-ffi' / 'include'}", f"-I{ROOT / 'e2e' / 'c'}"),
    bad_sample="int f( {\n",
    bad_suffix=".c",
)

JAVASCRIPT = Checker(
    tool="node",
    how="node --check",
    suffixes=(".js", ".mjs", ".cjs"),
    arguments=("--check",),
    bad_sample="function f( { }\n",
    bad_suffix=".js",
)

TYPESCRIPT = Checker(
    tool="tsc",
    how="tsc --noResolve, syntactic (TS1xxx) diagnostics only",
    suffixes=(".ts", ".mts", ".cts"),
    arguments=("--noEmit", "--ignoreConfig", "--skipLibCheck", "--noResolve"),
    syntactic_codes_only=True,
    bad_sample="export function f( : void {}\n",
    bad_suffix=".ts",
)

# `node --check` is deliberately absent for TypeScript: it exits 0 on
# syntactically invalid .ts, so it would report a silent pass. ~keep

NO_PARSE_ONLY_CSHARP = (
    "Roslyn ships no parse-only mode; csc always binds references, and unlike "
    "TypeScript its syntax diagnostics share the CS1xxx band with semantic ones, "
    "so they cannot be filtered apart. dotnet build would be a full restore."
)

NO_PARSE_ONLY_KOTLIN = (
    "kotlinc has no parse-only mode and resolves the Android SDK during "
    "compilation, so a full compile would fail for reasons unrelated to syntax."
)

# Maps each language Alef generates to its source roots and parse checkers. The
# key set is cross-checked against alef.toml at run time, so a language added to
# the generator cannot silently go unswept. ~keep
LANGUAGES: tuple[Language, ...] = (
    Language("python", ("packages/python", "e2e/python"), (PYTHON,)),
    Language("node", ("crates/html-to-markdown-node", "e2e/node"), (JAVASCRIPT, TYPESCRIPT)),
    Language("ruby", ("packages/ruby", "e2e/ruby"), (RUBY,)),
    Language("php", ("crates/html-to-markdown-php", "e2e/php", "test_apps/php"), (PHP,)),
    Language("ffi", ("crates/html-to-markdown-ffi/include", "e2e/c"), (C,)),
    Language("go", ("packages/go", "e2e/go"), (GO,)),
    Language("java", ("packages/java", "e2e/java"), (JAVA,)),
    Language(
        "csharp",
        ("packages/csharp", "e2e/csharp"),
        no_parse_only_reason=NO_PARSE_ONLY_CSHARP,
        unchecked_suffixes=(".cs",),
    ),
    Language("elixir", ("packages/elixir", "e2e/elixir"), (ELIXIR,)),
    Language("wasm", ("crates/html-to-markdown-wasm", "e2e/wasm"), (JAVASCRIPT, TYPESCRIPT)),
    Language("r", ("packages/r/R", "packages/r/tests", "e2e/r"), (R,)),
    Language(
        "kotlin_android",
        ("packages/kotlin-android", "e2e/kotlin_android"),
        (JAVA,),
        NO_PARSE_ONLY_KOTLIN,
        unchecked_suffixes=(".kt", ".kts"),
    ),
    Language("jni", ("crates/html-to-markdown-rs-jni",), requires_cargo=True, unchecked_suffixes=(".rs",)),
    Language("swift", ("packages/swift", "e2e/swift", "e2e/swift_e2e"), (SWIFT,)),
    Language("dart", ("packages/dart", "e2e/dart"), (DART,)),
    Language("zig", ("packages/zig/src", "packages/zig/examples", "packages/zig/build.zig", "e2e/zig"), (ZIG,)),
)

STATUS_PASS = "PASS"  # noqa: S105  # a result label, not a credential
STATUS_FAIL = "FAIL"
STATUS_PARTIAL = "PARTIAL"
STATUS_SKIPPED = "SKIPPED"
STATUS_NO_SOURCES = "NO SOURCES"


@dataclass
class CheckResult:
    """Outcome of one checker against one language's files."""

    checker: Checker
    files: int
    ran: bool
    passed: bool = False
    skip_reason: str = ""
    output: str = ""


@dataclass
class LanguageResult:
    """Aggregated outcome for one binding surface."""

    language: Language
    files: int = 0
    checks: list[CheckResult] = field(default_factory=list)

    @property
    def checked_files(self) -> int:
        """How many files were actually handed to the front end."""
        return sum(check.files for check in self.checks if check.ran)

    @property
    def status(self) -> str:
        """PASS / PARTIAL / SKIPPED / FAILED for this language."""
        if not self.checks:
            return STATUS_NO_SOURCES if self.files == 0 else STATUS_SKIPPED
        if any(check.ran and not check.passed for check in self.checks):
            return STATUS_FAIL
        ran = [check for check in self.checks if check.ran]
        if not ran:
            return STATUS_SKIPPED
        if len(ran) < len(self.checks) or self.checked_files < self.files:
            return STATUS_PARTIAL
        return STATUS_PASS

    @property
    def detail(self) -> str:
        """Human-readable explanation shown under the status line."""
        if self.status in {STATUS_SKIPPED, STATUS_NO_SOURCES}:
            reasons = {check.skip_reason for check in self.checks if check.skip_reason}
            if reasons:
                return "; ".join(sorted(reasons))
            return self.language.no_parse_only_reason or "no tracked sources found under the configured roots"
        how = ", ".join(sorted({check.checker.how for check in self.checks if check.ran}))
        if self.status == STATUS_PARTIAL:
            skipped = "; ".join(sorted({check.skip_reason for check in self.checks if check.skip_reason}))
            unchecked = self.files - self.checked_files
            note = skipped or f"{unchecked} file(s) have no parse-only checker"
            return f"{how} -- but {note}"
        return how


@cache
def tracked_files() -> tuple[Path, ...]:
    """Every git-tracked file, as absolute paths.

    Sourcing from git's index rather than walking the filesystem is what keeps
    ``packages/r``'s vendored crate tree and ``packages/zig``'s stdlib cache --
    thousands of files that are not our generated bindings -- out of the sweep.
    """
    result = subprocess.run(
        ["git", "-C", str(ROOT), "ls-files", "-z"],
        capture_output=True,
        check=True,
        text=True,
    )
    return tuple(ROOT / name for name in result.stdout.split("\0") if name)


def files_under(roots: tuple[str, ...], suffixes: tuple[str, ...] | None = None) -> list[Path]:
    """Tracked files under any of ``roots``, optionally filtered by suffix."""
    prefixes = [(ROOT / root) for root in roots]
    found = [
        path
        for path in tracked_files()
        if any(path == prefix or prefix in path.parents for prefix in prefixes)
        and (suffixes is None or path.suffix in suffixes)
    ]
    return sorted(found)


def alef_languages() -> list[str]:
    """The language list Alef generates for, read from alef.toml.

    Read rather than hardcoded so that a language added to the generator shows
    up here as an unmapped-language failure instead of quietly going unswept.
    """
    if not ALEF_CONFIG.is_file():
        return []
    text = ALEF_CONFIG.read_text(encoding="utf-8")
    match = re.search(r"^languages\s*=\s*\[(.*?)\]", text, re.MULTILINE | re.DOTALL)
    if match is None:
        return []
    return re.findall(r'"([^"]+)"', match.group(1))


def syntactic_failures(output: str) -> list[str]:
    """Lines carrying a TypeScript *syntactic* (TS1xxx) diagnostic."""
    failures = []
    for line in output.splitlines():
        match = TS_DIAGNOSTIC_RE.search(line)
        if match is not None and int(match.group(1)) < TS_SYNTACTIC_MAX_CODE:
            failures.append(line)
    return failures


def run_checker(checker: Checker, files: list[Path], helper: Path | None) -> tuple[bool, str]:
    """Run one checker over ``files``. Returns (passed, output)."""
    if checker.one_file_at_a_time and len(files) > 1:
        outputs: list[str] = []
        passed = True
        for path in files:
            file_passed, output = run_checker_once(checker, [path], helper)
            passed = passed and file_passed
            if not file_passed and output:
                outputs.append(output)
        return passed, "\n".join(outputs)
    return run_checker_once(checker, files, helper)


def run_checker_once(checker: Checker, files: list[Path], helper: Path | None) -> tuple[bool, str]:
    """Invoke a checker exactly once over ``files``."""
    try:
        completed = subprocess.run(
            checker.command(files, helper),
            capture_output=True,
            text=True,
            timeout=CHECK_TIMEOUT_SECONDS,
            cwd=ROOT,
            check=False,
        )
    except subprocess.TimeoutExpired:
        return False, f"{checker.tool} timed out after {CHECK_TIMEOUT_SECONDS}s"

    output = (completed.stdout + completed.stderr).strip()
    if checker.syntactic_codes_only:
        failures = syntactic_failures(output)
        return not failures, "\n".join(failures)
    if checker.failure_from_stderr:
        return not completed.stderr.strip(), completed.stderr.strip()
    return completed.returncode == 0, output


def materialise_java_helper(directory: Path) -> Path:
    path = directory / "ParseOnly.java"
    path.write_text(JAVA_PARSE_HELPER, encoding="utf-8")
    return path


def evaluate_language(language: Language, helper: Path | None, with_cargo: bool) -> LanguageResult:
    """Run every checker configured for one language."""
    result = LanguageResult(language=language)
    result.files = len(files_under(language.roots, language.source_suffixes))

    if language.requires_cargo:
        # cargo check is the only way to prove a Rust crate parses, and it is a
        # full type check: minutes of CPU and a lock shared with any concurrent
        # build. Opt-in so the cheap sweep stays cheap. ~keep
        crate_files = files_under(language.roots, (".rs",))
        if not with_cargo:
            result.checks.append(
                CheckResult(
                    checker=Checker(tool="cargo", how="cargo check", suffixes=(".rs",)),
                    files=len(crate_files),
                    ran=False,
                    skip_reason="cargo check not run (pass --with-cargo; it is a full type check, not a parse)",
                )
            )
            return result
        passed, output = run_checker(
            Checker(tool="cargo", how="cargo check", suffixes=(".rs",), arguments=("check", "--all-targets")),
            [],
            None,
        )
        result.checks.append(
            CheckResult(
                checker=Checker(tool="cargo", how="cargo check", suffixes=(".rs",)),
                files=len(crate_files),
                ran=True,
                passed=passed,
                output=output,
            )
        )
        return result

    for checker in language.checkers:
        files = files_under(language.roots, checker.suffixes)
        if not files:
            continue
        if shutil.which(checker.tool) is None:
            result.checks.append(
                CheckResult(
                    checker=checker,
                    files=len(files),
                    ran=False,
                    skip_reason=f"{checker.tool} not found on PATH",
                )
            )
            continue
        passed, output = run_checker(checker, files, helper)
        result.checks.append(CheckResult(checker=checker, files=len(files), ran=True, passed=passed, output=output))

    if language.no_parse_only_reason and result.checked_files < result.files:
        result.checks.append(
            CheckResult(
                checker=Checker(tool="(none)", how="n/a", suffixes=()),
                files=result.files - result.checked_files,
                ran=False,
                skip_reason=language.no_parse_only_reason,
            )
        )
    return result


def self_test(helper: Path | None) -> int:
    """Feed every checker a broken sample and assert it rejects it."""
    print("=" * 78)
    print("Self-test: does each checker actually reject invalid syntax?")
    print("=" * 78)

    checkers = {checker.tool: checker for language in LANGUAGES for checker in language.checkers}
    proven = 0
    unproven = 0
    skipped = 0
    with tempfile.TemporaryDirectory() as directory:
        for tool, checker in sorted(checkers.items()):
            if not checker.bad_sample:
                continue
            if shutil.which(checker.tool) is None:
                skipped += 1
                print(f"  SKIPPED  {tool:<8} {checker.tool} not found on PATH")
                continue
            name = "ParseOnly" if checker.needs_java_helper else "broken"
            sample = Path(directory) / f"{name}{checker.bad_suffix}"
            sample.write_text(checker.bad_sample, encoding="utf-8")
            passed, _ = run_checker(checker, [sample], helper)
            if passed:
                unproven += 1
                print(f"  UNPROVEN {tool:<8} accepted invalid syntax -- this checker proves nothing")
            else:
                proven += 1
                print(f"  PROVEN   {tool:<8} rejected invalid syntax ({checker.how})")

    print("\n" + "=" * 78)
    print(f"{proven + unproven + skipped} checkers, {proven} proven, {unproven} unproven, {skipped} skipped")
    print("FAIL" if unproven else "OK")
    print("=" * 78)
    return EXIT_FAILED if unproven else EXIT_OK


def report(results: list[LanguageResult], unmapped: list[str], verbose: bool) -> None:
    print("=" * 78)
    print("Binding parse sweep")
    print("=" * 78)
    print()

    for result in results:
        print(f"  {result.status:<11} {result.language.name:<15} {result.checked_files}/{result.files} files")
        print(f"              {result.detail}")
        for check in result.checks:
            if check.ran and not check.passed:
                lines = check.output.splitlines()
                shown = lines if verbose else lines[:10]
                for line in shown:
                    print(f"                {line}")
                if len(shown) < len(lines):
                    print(f"                ... and {len(lines) - len(shown)} more (--verbose to list)")

    if unmapped:
        print("\nLanguages in alef.toml with no checker mapping (coverage hole):")
        for name in unmapped:
            print(f"  UNMAPPED  {name}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--strict", action="store_true", help="exit nonzero when any language is skipped")
    parser.add_argument("--self-test", action="store_true", help="prove each checker rejects invalid syntax, then exit")
    parser.add_argument("--with-cargo", action="store_true", help="include cargo check for the Rust crates (slow)")
    parser.add_argument("-v", "--verbose", action="store_true", help="print every diagnostic line")
    parser.add_argument("--json", action="store_true", help="emit a machine-readable summary on stdout")
    args = parser.parse_args()

    with tempfile.TemporaryDirectory() as directory:
        helper = materialise_java_helper(Path(directory))

        if args.self_test:
            return self_test(helper)

        results = [evaluate_language(language, helper, args.with_cargo) for language in LANGUAGES]

    mapped = {language.name for language in LANGUAGES}
    unmapped = [name for name in alef_languages() if name not in mapped]

    counts = {
        status: sum(1 for result in results if result.status == status)
        for status in (STATUS_PASS, STATUS_FAIL, STATUS_PARTIAL, STATUS_SKIPPED, STATUS_NO_SOURCES)
    }

    if args.json:
        print(
            json.dumps(
                {
                    "languages": [
                        {
                            "name": result.language.name,
                            "status": result.status,
                            "files": result.files,
                            "checked_files": result.checked_files,
                            "detail": result.detail,
                        }
                        for result in results
                    ],
                    "unmapped": unmapped,
                    "counts": counts,
                },
                indent=2,
            )
        )

    report(results, unmapped, args.verbose)

    checked_files = sum(result.checked_files for result in results)
    total_files = sum(result.files for result in results)
    not_proven = counts[STATUS_SKIPPED] + counts[STATUS_PARTIAL] + counts[STATUS_NO_SOURCES]

    print("\n" + "=" * 78)
    print(
        f"{len(results)} languages checked, {counts[STATUS_PASS]} passed, "
        f"{counts[STATUS_PARTIAL]} partial, "
        f"{counts[STATUS_SKIPPED]} skipped (no toolchain or no parse-only mode), "
        f"{counts[STATUS_NO_SOURCES]} without sources, {counts[STATUS_FAIL]} failed"
    )
    print(f"{checked_files}/{total_files} tracked source files actually parsed")

    failed = counts[STATUS_FAIL] > 0 or bool(unmapped)
    if failed:
        print("FAIL")
        print("=" * 78)
        return EXIT_FAILED
    if args.strict and not_proven:
        print(f"FAIL (--strict: {not_proven} language(s) not fully proven)")
        print("=" * 78)
        return EXIT_SKIPPED_STRICT
    print("OK" if not not_proven else f"OK ({not_proven} language(s) not fully proven -- see above)")
    print("=" * 78)
    return EXIT_OK


if __name__ == "__main__":
    raise SystemExit(main())
