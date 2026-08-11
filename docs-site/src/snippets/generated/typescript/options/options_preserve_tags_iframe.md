---
id: fixture_node_options_preserve_tags_iframe
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { preserveTags: ["iframe"] };
  const result = convert("<p>Before</p><iframe src='video.html' width='560'></iframe><p>After</p>", options);
}

void main();

```
