```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.extractMetadata = true; return _u0; })();
  const result = convert("<html><head><title>Article</title></head><body><article itemscope itemtype=\"https://schema.org/Article\"><h1 itemprop=\"headline\">Breaking News Today</h1><span itemprop=\"author\">Jane Reporter</span><span itemprop=\"datePublished\">2024-04-22</span><div itemprop=\"articleBody\"><p>The article content goes here with important information about the breaking news story.</p></div></article></body></html>", options);
}

void main();

```
