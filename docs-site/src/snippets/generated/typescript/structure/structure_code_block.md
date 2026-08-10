```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { includeDocumentStructure: true };
  const result = convert("<p>Example code:</p><pre><code class=\"language-rust\">fn main() { println!(\"Hello\"); }</code></pre>", options);
}

void main();

```
