---
id: fixture_node_visitor_horizontal_rule_custom
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
    visitHorizontalRule(ctx: any): string | { Custom: string } {
        return { Custom: "\n[DIVIDER]\n" };
    },

    }

  const result = convert("<h1>Section A</h1><p>Content A</p><hr><h1>Section B</h1><p>Content B</p>", { visitor: _testVisitor as any });
}

void main();

```
