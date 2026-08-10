```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<table><tr><td>Product</td><td>Price</td></tr><tr><td>Apple</td><td>1.00</td></tr></table>", undefined);
}

void main();

```
