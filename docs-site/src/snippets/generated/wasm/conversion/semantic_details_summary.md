```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<details><summary>Click to expand</summary><p>Hidden content here.</p></details>", undefined);
}

void main();

```
