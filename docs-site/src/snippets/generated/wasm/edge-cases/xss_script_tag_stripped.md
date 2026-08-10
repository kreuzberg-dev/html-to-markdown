```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>Safe content.</p><script>alert('xss')</script><p>More safe content.</p>", undefined);
}

void main();

```
