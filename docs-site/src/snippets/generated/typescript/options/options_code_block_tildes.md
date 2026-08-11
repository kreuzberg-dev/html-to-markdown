---
id: fixture_node_options_code_block_tildes
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
  const result = convert("<pre><code>let x = 1;</code></pre>", options);
}

void main();

```
