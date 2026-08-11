---
id: fixture_node_visitor_custom_image
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
    visitImage(ctx: any, src: any, alt: any, title: any): string | { Custom: string } {
        return { Custom: `[Image: ${alt}]` };
    },

    }

  const result = convert("<img src=\"banner.png\" alt=\"Banner\">", { visitor: _testVisitor as any });
}

void main();

```
