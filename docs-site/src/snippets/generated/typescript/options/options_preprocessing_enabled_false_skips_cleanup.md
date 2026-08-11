---
id: fixture_node_options_preprocessing_enabled_false_skips_cleanup
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { preprocessing: { enabled: false } };
  const result = convert("<nav>NavSection</nav><p>Paragraph</p>", options);
}

void main();

```
