```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<a href=\"https://example.com\"><strong>Bold link</strong></a>", undefined);
}

void main();

```
