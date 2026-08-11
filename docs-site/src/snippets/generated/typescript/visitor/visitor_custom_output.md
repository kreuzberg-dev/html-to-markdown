---
id: fixture_node_visitor_custom_output
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
        return { Custom: "## REPLACED HEADING" };
    },

    }

  const result = convert("<h1>Original Heading</h1>", { visitor: _testVisitor as any });
}

void main();

```
