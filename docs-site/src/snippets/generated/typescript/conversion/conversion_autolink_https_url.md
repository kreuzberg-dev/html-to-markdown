---
id: fixture_node_conversion_autolink_https_url
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<a href=\"https://example.com\">https://example.com</a>", undefined);
}

void main();

```
