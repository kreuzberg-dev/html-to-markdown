```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>H<sub>2</sub>O and E=mc<sup>2</sup></p>", undefined);
}

void main();

```
