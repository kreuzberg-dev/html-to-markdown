---
id: fixture_node_visitor_underline_custom
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
    visitUnderline(ctx: any, text: any): string | { Custom: string } {
        return { Custom: `_${text}_` };
    },

    }

  const result = convert("<p>This is <u>very important</u> text.</p>", { visitor: _testVisitor as any });
}

void main();

```
