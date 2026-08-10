```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<h1>One</h1>\n<!-- /// --->\n<p>Two</p>", undefined);
}

void main();

```
