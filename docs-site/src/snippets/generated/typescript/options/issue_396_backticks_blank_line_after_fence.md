```typescript title="TypeScript"
import { CodeBlockStyle, ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { codeBlockStyle: CodeBlockStyle.Backticks };
  const result = convert("<p>Foo</p><pre><code>1\n2\n</code></pre><p>Bar</p>", options);
}

void main();

```
