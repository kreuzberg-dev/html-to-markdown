```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<ul><li><input type=\"checkbox\" checked> Done task</li><li><input type=\"checkbox\"> Pending task</li></ul>", undefined);
}

void main();

```
