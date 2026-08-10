```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<article><h2>Article Title</h2><p>Article body.</p></article>", undefined);
}

void main();

```
