---
id: fixture_node_visitor_link_bare_string_preserves_case
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
        return `[${text}](https://new-cdn.com/file.pdf)`;
    },

    }

  const result = convert("<a href=\"https://old-cdn.com/file.pdf\">Download</a>", { visitor: _testVisitor as any });
}

void main();

```
