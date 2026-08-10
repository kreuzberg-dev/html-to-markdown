```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<div><div><p>Nested text</p></div></div>", undefined);
}

void main();

```
