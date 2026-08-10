```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>Copyright: &#169; Trade: &#174; Euro: &#8364; Hex: &#x00A9;</p>", undefined);
}

void main();

```
