```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<img src=\"photo.jpg\" alt=\"A photo\">", undefined);
}

void main();

```
