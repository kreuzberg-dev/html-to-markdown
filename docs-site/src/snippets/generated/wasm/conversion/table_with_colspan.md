```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<table><thead><tr><th colspan=\"2\">Full Name</th></tr></thead><tbody><tr><td>John</td><td>Doe</td></tr></tbody></table>", undefined);
}

void main();

```
