```typescript title="TypeScript"
import { CodeBlockStyle, ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { codeBlockStyle: CodeBlockStyle.Indented };
  const result = convert("<pre><code>print('hello')</code></pre>", options);
}

void main();

```
