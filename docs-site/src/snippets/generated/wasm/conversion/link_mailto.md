```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<a href=\"mailto:user@example.com\">Email us</a>", undefined);
}

void main();

```
