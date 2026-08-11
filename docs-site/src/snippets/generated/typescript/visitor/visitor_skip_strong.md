---
id: fixture_node_visitor_skip_strong
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitStrong(ctx: any, text: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<p>Normal <strong>bold text</strong> normal</p>", { visitor: _testVisitor as any });
}

void main();

```
