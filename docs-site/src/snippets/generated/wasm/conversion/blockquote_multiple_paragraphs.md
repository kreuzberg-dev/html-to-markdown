```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<blockquote><p>First paragraph.</p><p>Second paragraph.</p></blockquote>", undefined);
}

void main();

```
