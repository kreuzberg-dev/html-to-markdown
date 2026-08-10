```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<section><h3>Section Heading</h3><p>Section content.</p></section>", undefined);
}

void main();

```
