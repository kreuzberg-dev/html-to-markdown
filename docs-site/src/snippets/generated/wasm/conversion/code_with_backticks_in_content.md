```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>Use <code>`backtick` here</code> carefully.</p>", undefined);
}

void main();

```
