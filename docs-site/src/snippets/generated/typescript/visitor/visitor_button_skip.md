---
id: fixture_node_visitor_button_skip
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
    visitButton(ctx: any, text: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<p>Actions available: <button>Save</button> <button>Delete</button> <button>Export</button></p>", { visitor: _testVisitor as any });
}

void main();

```
