```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<ol><li>Step 1<ol><li>Step 1a</li><li>Step 1b</li></ol></li><li>Step 2</li></ol>", undefined);
}

void main();

```
