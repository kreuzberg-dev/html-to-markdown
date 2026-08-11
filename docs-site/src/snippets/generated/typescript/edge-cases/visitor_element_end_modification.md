---
id: fixture_node_visitor_element_end_modification
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
    visitElementEnd(ctx: any, output: any): string | { Custom: string } {
        return { Custom: "MODIFIED OUTPUT" };
    },

    }

  const result = convert("<blockquote><p>Original quote</p></blockquote>", { visitor: _testVisitor as any });
}

void main();

```
