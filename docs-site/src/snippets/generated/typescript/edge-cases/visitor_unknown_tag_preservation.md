---
id: fixture_node_visitor_unknown_tag_preservation
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

  const result = convert("<article><p>Article text</p><x-custom>Custom element with content</x-custom><p>More article text</p></article>", { visitor: _testVisitor as any });
}

void main();

```
