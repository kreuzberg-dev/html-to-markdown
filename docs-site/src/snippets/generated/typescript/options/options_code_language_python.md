```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { codeLanguage: "python" };
  const result = convert("<pre><code>def hello(): pass</code></pre>", options);
}

void main();

```
