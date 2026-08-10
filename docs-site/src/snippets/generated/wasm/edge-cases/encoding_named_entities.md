```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>Em dash&mdash;used for parenthetical remarks&mdash;is common. Ellipsis&hellip; indicates omission. Non-breaking&nbsp;space.</p>", undefined);
}

void main();

```
