```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<a href=\"foobar.png\">foobar.png</a> <a href=\"https://www.heise.de\">https://www.heise.de</a>", undefined);
}

void main();

```
