```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<pre><code class=\"language-python\">print('hello')</code></pre>", undefined);
}

void main();

```
