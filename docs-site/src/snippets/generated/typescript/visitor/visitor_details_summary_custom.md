---
id: fixture_node_visitor_details_summary_custom
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
    visitSummary(ctx: any, text: any): string | { Custom: string } {
        return { Custom: `[EXPANDABLE] ${text}` };
    },

    }

  const result = convert("<details><summary>Click to expand</summary><p>This content is initially hidden.</p><p>But can be revealed by the user.</p></details>", { visitor: _testVisitor as any });
}

void main();

```
