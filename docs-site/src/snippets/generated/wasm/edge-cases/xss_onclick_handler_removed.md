```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p><a href=\"https://example.com\" onclick=\"alert('xss')\">Click me</a></p><button onmouseover=\"steal_data()\">Hover me</button>", undefined);
}

void main();

```
