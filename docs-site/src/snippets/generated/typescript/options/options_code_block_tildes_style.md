---
id: fixture_node_options_code_block_tildes_style
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { CodeBlockStyle, ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { codeBlockStyle: CodeBlockStyle.Tildes };
  const result = convert("<pre><code>some code</code></pre>", options);
}

void main();

```
