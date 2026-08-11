---
id: fixture_node_visitor_skip_links
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
        return "Skip";
    },

    }

  const result = convert("<p>Before <a href=\"https://example.com\">link text</a> after</p>", { visitor: _testVisitor as any });
}

void main();

```
