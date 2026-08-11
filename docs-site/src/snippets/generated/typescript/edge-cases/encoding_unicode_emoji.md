---
id: fixture_node_encoding_unicode_emoji
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p>Hello 🌍 World 🚀</p><p>Stars: ⭐ ✨</p>", undefined);
}

void main();

```
