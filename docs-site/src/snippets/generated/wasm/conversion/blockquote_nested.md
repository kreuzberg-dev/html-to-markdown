```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<blockquote><p>Outer quote.</p><blockquote><p>Inner quote.</p></blockquote></blockquote>", undefined);
}

void main();

```
