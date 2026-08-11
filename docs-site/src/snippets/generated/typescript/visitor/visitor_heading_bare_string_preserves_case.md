---
id: fixture_node_visitor_heading_bare_string_preserves_case
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
        return `## ${text} ##`;
    },

    }

  const result = convert("<h2>Important Section Title</h2><p>Body.</p>", { visitor: _testVisitor as any });
}

void main();

```
