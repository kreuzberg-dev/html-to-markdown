---
id: fixture_node_visitor_custom_link_static
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
    visitLink(ctx: any, href: any, text: any, title: any): string | { Custom: string } {
        return { Custom: "[REDACTED LINK]" };
    },

    }

  const result = convert("<a href=\"https://example.com\">Click here</a>", { visitor: _testVisitor as any });
}

void main();

```
