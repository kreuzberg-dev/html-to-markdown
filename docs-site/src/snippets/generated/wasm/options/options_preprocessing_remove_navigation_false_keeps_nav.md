---
id: fixture_wasm_options_preprocessing_remove_navigation_false_keeps_nav
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmConversionOptions, WasmPreprocessingOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.preprocessing = (() => { const _u1 = WasmPreprocessingOptions.default(); _u1.removeNavigation = false; return _u1; })(); return _u0; })();
  const result = convert("<nav>SiteMenu</nav><main><p>MainContent</p></main><aside>SidebarText</aside>", options);
}

void main();

```
