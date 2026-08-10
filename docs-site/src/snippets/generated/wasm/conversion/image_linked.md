```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<a href=\"https://example.com\"><img src=\"icon.png\" alt=\"Icon\"></a>", undefined);
}

void main();

```
