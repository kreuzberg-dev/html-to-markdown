```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<ul><li>Item A<ol><li>Sub 1</li><li>Sub 2</li></ol></li><li>Item B</li></ul>", undefined);
}

void main();

```
