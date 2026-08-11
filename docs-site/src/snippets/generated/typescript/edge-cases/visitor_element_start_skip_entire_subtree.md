---
id: fixture_node_visitor_element_start_skip_entire_subtree
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
    visitElementStart(ctx: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<div><h1>Title</h1><p>Content</p></div>", { visitor: _testVisitor as any });
}

void main();

```
