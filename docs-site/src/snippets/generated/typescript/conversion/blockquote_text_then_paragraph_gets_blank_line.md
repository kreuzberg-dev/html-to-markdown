---
id: fixture_node_blockquote_text_then_paragraph_gets_blank_line
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<blockquote>Just text, then <p>a paragraph</p></blockquote>", undefined);
}

void main();

```
