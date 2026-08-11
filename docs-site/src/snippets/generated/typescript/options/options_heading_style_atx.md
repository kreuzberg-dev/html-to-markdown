---
id: fixture_node_options_heading_style_atx
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, HeadingStyle, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { headingStyle: HeadingStyle.Atx };
  const result = convert("<h1>Title</h1><h2>Subtitle</h2>", options);
}

void main();

```
