```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<!-- This is a comment --><!-- Another comment -->", undefined);
}

void main();

```
