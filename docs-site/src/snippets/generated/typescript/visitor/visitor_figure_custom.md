---
id: fixture_node_visitor_figure_custom
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
    visitFigcaption(ctx: any, text: any): string | { Custom: string } {
        return { Custom: `*${text}*` };
    },

    }

  const result = convert("<article><h1>Article Title</h1><p>Introduction paragraph.</p><figure><img src=\"diagram.png\" alt=\"System architecture diagram\"><figcaption>Figure 1: System Architecture</figcaption></figure><p>Explanation of the figure.</p></article>", { visitor: _testVisitor as any });
}

void main();

```
