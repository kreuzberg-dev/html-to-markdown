---
id: fixture_node_visitor_preserve_html
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
    visitCustomElement(ctx: any, tagName: any, html: any): string | { Custom: string } {
        return "PreserveHtml";
    },

    }

  const result = convert("<div><custom-tag>Custom content</custom-tag></div>", { visitor: _testVisitor as any });
}

void main();

```
