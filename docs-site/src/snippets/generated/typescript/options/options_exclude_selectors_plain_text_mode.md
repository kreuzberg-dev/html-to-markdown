---
id: fixture_node_options_exclude_selectors_plain_text_mode
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, OutputFormat, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { excludeSelectors: [".nav"], outputFormat: OutputFormat.Plain };
  const result = convert("<body><div class=\"nav\">Navigation</div><p>Article body</p></body>", options);
}

void main();

```
