// Static guard on the committed `index.js` loader.
//
// `index.js` is a committed source file, not a build artifact: the publish pipeline
// copies it verbatim (publish.yaml stages it into `typescript-defs/`, then back over
// the crate dir before `npm publish`), and `napi build --release` — the command that
// staging step runs — does not rewrite it at all. Whatever is committed is what ships.
//
// That makes a stray `napi build` run from the *repo root* silently destructive: the
// root manifest is `html-to-markdown-monorepo` with the default binary name `index`,
// so the emitted loader requires `html-to-markdown-monorepo-*` packages that are never
// published and `./index.*.node` files that are never shipped. It happened in
// a7d66810a and nothing caught it, because unlike `index.d.ts` (freshness-gated by
// `alef verify`) nothing gated `index.js`.
//
// Unlike `verify-esm-exports.mjs`, this check is purely static — it never loads the
// native binding — so it runs anywhere, including where nothing has been built.

import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const crateDir = join(dirname(fileURLToPath(import.meta.url)), "..");
const loaderPath = join(crateDir, "index.js");
const loaderSource = readFileSync(loaderPath, "utf8");
const manifest = JSON.parse(readFileSync(join(crateDir, "package.json"), "utf8"));

const binaryName = manifest.napi?.binaryName;
if (!binaryName) {
  throw new Error("package.json is missing napi.binaryName");
}

const declaredOptionalDeps = Object.keys(manifest.optionalDependencies ?? {});
if (declaredOptionalDeps.length === 0) {
  throw new Error("package.json declares no optionalDependencies to verify against");
}

// Scan every string literal rather than only `require("...")` arguments: the committed
// loader keeps its platform names in a lookup table and calls `require(variable)`, while
// raw NAPI-RS output inlines them into `require("...")`. Both shapes must be checked, since
// the whole point is to detect the second one replacing the first. ~keep
const PLATFORM_TOKENS = new Set(["linux", "darwin", "win32", "android", "freebsd", "openharmony", "wasi"]);

// Module specifiers never contain whitespace, and forbidding it here keeps the scan
// self-realigning: an apostrophe in prose (NAPI-RS output has several) would otherwise
// open a bogus "string" that swallows the real specifiers up to the next apostrophe. ~keep
const stringLiterals = [...loaderSource.matchAll(/["']([^"'\s]+)["']/g)].map((match) => match[1]);

const localBinaryPaths = new Set();
const referencedPackages = new Set();
for (const literal of stringLiterals) {
  if (literal.startsWith(".")) {
    if (literal.endsWith(".node")) {
      localBinaryPaths.add(literal);
    }
    continue;
  }
  // The loader reads `<pkg>/package.json` to report the resolved version. ~keep
  const packageName = literal.replace(/\/package\.json$/, "");
  if (!/^(@[^/\s]+\/)?[^/\s]+$/.test(packageName)) {
    continue;
  }
  // A platform package is `<packageName>-<target>`, so the platform token always has a
  // package prefix before it. That prefix is what distinguishes the package name
  // `@scope/pkg-darwin-arm64` from the loader table's bare `"darwin"` / `"linux-x64-gnu"`. ~keep
  const segments = packageName.split("/").pop().split("-");
  const platformIndex = segments.findIndex((segment) => PLATFORM_TOKENS.has(segment));
  if (platformIndex > 0) {
    referencedPackages.add(packageName);
  }
}

const failures = [];

// 1. The loader must reference exactly the platform packages the manifest declares.
//    Set equality (not subset) catches both invented names and dropped platforms.
const sortedJoin = (values) => [...values].sort().join("\n    ");
const unexpected = [...referencedPackages].filter((name) => !declaredOptionalDeps.includes(name));
const missing = declaredOptionalDeps.filter((name) => !referencedPackages.has(name));
if (unexpected.length > 0) {
  failures.push(
    `index.js requires platform packages that are not declared in optionalDependencies ` +
      `(never published, so the loader can never resolve them):\n    ${sortedJoin(unexpected)}`,
  );
}
if (missing.length > 0) {
  failures.push(
    `index.js never requires these declared optionalDependencies, so those platforms ` +
      `cannot load:\n    ${sortedJoin(missing)}`,
  );
}

// 2. Local `.node` paths must be named after napi.binaryName, which is what the
//    platform packages actually contain.
const expectedPrefix = `./${binaryName}.`;
const misnamed = [...localBinaryPaths].filter((path) => !path.startsWith(expectedPrefix));
if (localBinaryPaths.size === 0) {
  failures.push("index.js requires no local .node file at all");
} else if (misnamed.length > 0) {
  failures.push(
    `index.js requires local .node files that do not match napi.binaryName ` +
      `("${binaryName}"):\n    ${sortedJoin(misnamed)}`,
  );
}

// 3. Every name index.d.ts declares must be re-exported for ESM named imports.
//    `module.exports = nativeBinding` alone does not give Node's CJS named-export
//    detection anything to see, which is what issue #450 was.
const declarationSource = readFileSync(join(crateDir, "index.d.ts"), "utf8");
const declaredExports = new Set(
  [...declarationSource.matchAll(/^export\s+(?:declare\s+)?(?:function|const|enum|class)\s+([A-Za-z_$][\w$]*)/gm)].map(
    (match) => match[1],
  ),
);
const reExported = new Set(
  [...loaderSource.matchAll(/^module\.exports\.([A-Za-z_$][\w$]*)\s*=/gm)].map((match) => match[1]),
);
if (declaredExports.size === 0) {
  failures.push("index.d.ts declares no exports — the export check would be vacuous");
}
const notReExported = [...declaredExports].filter((name) => !reExported.has(name));
if (notReExported.length > 0) {
  failures.push(
    `index.d.ts declares these exports but index.js never re-exports them, so ` +
      `\`import { ... }\` fails at runtime:\n    ${sortedJoin(notReExported)}`,
  );
}

if (failures.length > 0) {
  console.error(`verify-loader-manifest: ${failures.length} problem(s) in ${loaderPath}\n`);
  for (const failure of failures) {
    console.error(`  - ${failure}\n`);
  }
  console.error(
    "If index.js looks like raw NAPI-RS output referencing `html-to-markdown-monorepo-*`,\n" +
      "a `napi build` was run from the repo root and clobbered the committed loader.\n" +
      "Restore it with: git show HEAD:crates/html-to-markdown-node/index.js > crates/html-to-markdown-node/index.js\n",
  );
  process.exit(1);
}

console.log(
  `verify-loader-manifest: index.js matches package.json ` +
    `(${declaredOptionalDeps.length} platform packages, binaryName "${binaryName}", ` +
    `${declaredExports.size} declared exports re-exported)`,
);
