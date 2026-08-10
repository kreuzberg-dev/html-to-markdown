```typescript title="TypeScript"
import { CodeBlockStyle, ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { codeBlockStyle: CodeBlockStyle.Backticks };
  const result = convert("<pre><code class=\"language-js\">console.log('hi');</code></pre>", options);
}

void main();

```
