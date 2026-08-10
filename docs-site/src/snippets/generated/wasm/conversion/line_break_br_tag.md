```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>First line.<br>Second line.</p>", undefined);
}

void main();

```
