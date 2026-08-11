---
id: fixture_node_visitor_form_custom
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
    visitForm(ctx: any, actionUrl: any, method: any): string | { Custom: string } {
        return { Custom: "[FORM PLACEHOLDER]" };
    },

    }

  const result = convert("<div><form action=\"/submit\" method=\"POST\"><label>Name: <input type=\"text\" name=\"name\"></label><button type=\"submit\">Submit</button></form></div>", { visitor: _testVisitor as any });
}

void main();

```
