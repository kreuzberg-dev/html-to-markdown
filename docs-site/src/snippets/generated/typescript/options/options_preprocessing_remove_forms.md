---
id: fixture_node_options_preprocessing_remove_forms
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { preprocessing: { removeForms: true } };
  const result = convert("<p>Before</p><form><input type='text'/><button>Submit</button></form><p>After</p>", options);
}

void main();

```
