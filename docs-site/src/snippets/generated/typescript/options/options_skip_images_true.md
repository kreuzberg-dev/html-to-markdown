---
id: fixture_node_options_skip_images_true
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { skipImages: true };
  const result = convert("<p>Before <img src='test.jpg' alt='photo'> After</p>", options);
}

void main();

```
