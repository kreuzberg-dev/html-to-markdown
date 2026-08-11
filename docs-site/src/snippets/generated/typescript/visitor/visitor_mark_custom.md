---
id: fixture_node_visitor_mark_custom
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
    visitMark(ctx: any, text: any): string | { Custom: string } {
        return { Custom: `==${text}==` };
    },

    }

  const result = convert("<p>This is a <mark>highlighted passage</mark> in the text.</p>", { visitor: _testVisitor as any });
}

void main();

```
