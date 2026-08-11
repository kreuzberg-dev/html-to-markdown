---
id: fixture_node_options_output_format_djot
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, OutputFormat, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { outputFormat: OutputFormat.Djot };
  const result = convert("<p>Simple paragraph.</p>", options);
}

void main();

```
