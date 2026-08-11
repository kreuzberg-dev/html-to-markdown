---
id: fixture_node_visitor_subscript_custom
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
    visitSubscript(ctx: any, text: any): string | { Custom: string } {
        return { Custom: `~${text}~` };
    },

    }

  const result = convert("<p>H<sub>2</sub>O is water.</p>", { visitor: _testVisitor as any });
}

void main();

```
