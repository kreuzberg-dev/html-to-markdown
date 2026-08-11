---
id: fixture_node_visitor_figure_custom_wrap
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
    visitFigureEnd(ctx: any, output: any): string | { Custom: string } {
        return { Custom: `${output}
[/FIGURE]
` };
    },

    visitFigureStart(ctx: any): string | { Custom: string } {
        return { Custom: "\n[FIGURE]\n" };
    },

    }

  const result = convert("<section><h2>Gallery</h2><figure><img src=\"photo1.jpg\" alt=\"Photo\"><figcaption>Beautiful sunset</figcaption></figure></section>", { visitor: _testVisitor as any });
}

void main();

```
