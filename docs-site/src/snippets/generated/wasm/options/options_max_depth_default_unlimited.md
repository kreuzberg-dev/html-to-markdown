```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<div><div><div><div><p>Deep content</p></div></div></div></div>", undefined);
}

void main();

```
