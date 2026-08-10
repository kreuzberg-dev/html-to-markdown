```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<html><head><style>body { color: red; }</style></head><body><style>.foo { margin: 0; }</style></body></html>", undefined);
}

void main();

```
