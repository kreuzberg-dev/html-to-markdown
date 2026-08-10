```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<a href=\"mailto:a@b.com\">a@b.com</a>", undefined);
}

void main();

```
