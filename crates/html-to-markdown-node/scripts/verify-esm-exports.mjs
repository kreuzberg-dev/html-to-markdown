// Build-time regression guard for issue #450.
//
// Loads the freshly built package through a real Node ESM *named* import — the
// exact shape that failed before the `fix-cjs-named-exports` pass ran. If the
// CJS named re-exports are ever missing again, this throws and fails the build
// instead of silently shipping a package whose `import { convert }` is broken.
//
// Host-only: it must actually load the native `.node`, so it belongs in the
// local/e2e `build` (which builds for the host via `--platform`), never in the
// cross-compiling publish pipeline.

import {convert, OutputFormat} from "../index.js";

if (typeof convert !== "function") {
  throw new Error(`#450 regression: named ESM export 'convert' is ${
      typeof convert}, expected function`);
}
if (typeof OutputFormat !== "object" || OutputFormat === null) {
  throw new Error(
      `#450 regression: named ESM export 'OutputFormat' did not resolve to an enum object`);
}

const result = convert("<h1>Hi</h1>");
const markdown = result.markdown ?? result.content;
if (markdown !== "# Hi\n") {
  throw new Error(`#450 regression: convert('<h1>Hi</h1>') returned ${
      JSON.stringify(markdown)}, expected "# Hi\\n"`);
}

console.log(
    "verify-esm-exports: named ESM import of convert/OutputFormat works");
