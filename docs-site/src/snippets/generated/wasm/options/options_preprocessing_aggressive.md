```typescript title="WebAssembly"
import { WasmConversionOptions, WasmPreprocessingOptions, WasmPreprocessingPreset, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.preprocessing = (() => { const _u1 = WasmPreprocessingOptions.default(); _u1.preset = WasmPreprocessingPreset.Aggressive; return _u1; })(); return _u0; })();
  const result = convert("<nav>Menu</nav><article><h1>Title</h1><p>Content</p></article><aside>Sidebar</aside><footer>Footer</footer>", options);
}

void main();

```
