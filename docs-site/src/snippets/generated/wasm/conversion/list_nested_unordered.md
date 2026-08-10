```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<ul><li>Parent A<ul><li>Child A1</li><li>Child A2</li></ul></li><li>Parent B</li></ul>", undefined);
}

void main();

```
