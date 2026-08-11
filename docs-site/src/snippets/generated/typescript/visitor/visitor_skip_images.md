---
id: fixture_node_visitor_skip_images
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
        return "Skip";
    },

    }

  const result = convert("<p>Before image</p><img src=\"photo.jpg\" alt=\"A photo\"><p>After image</p>", { visitor: _testVisitor as any });
}

void main();

```
