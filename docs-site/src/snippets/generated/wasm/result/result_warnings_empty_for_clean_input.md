```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<h1>Title</h1><p>Clean content with <a href='https://example.com'>a link</a>.</p>", undefined);
}

void main();

```
