---
target: wasm
---

```javascript
import { convert, WasmConversionOptions } from "@xberg-io/html-to-markdown-wasm";

const html =
  '<html><head><title>My Page</title></head><body><h1>Hello</h1><a href="https://example.com">Link</a></body></html>';

const options = WasmConversionOptions.default();
options.extractMetadata = true;
const result = convert(html, options);

console.log("Markdown:", result.content);
console.log("Title:", result.metadata.document.title);
console.log(
  "Links:",
  result.metadata.links.map((link) => link.href),
);
```
