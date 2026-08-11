---
id: fixture_node_visitor_custom_heading
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
    visitHeading(ctx: any, level: any, text: any, id: any): string | { Custom: string } {
        return { Custom: `--- ${text} ---` };
    },

    }

  const result = convert("<h2>Section Title</h2><p>Content below heading.</p>", { visitor: _testVisitor as any });
}

void main();

```
