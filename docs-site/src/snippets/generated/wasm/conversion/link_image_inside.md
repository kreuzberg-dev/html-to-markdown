```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<a href=\"https://example.com\"><img src=\"logo.png\" alt=\"Logo\"></a>", undefined);
}

void main();

```
