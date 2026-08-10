```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<table><tr><th>X</th></tr><tr><td>Y</td></tr></table>", undefined);
}

void main();

```
