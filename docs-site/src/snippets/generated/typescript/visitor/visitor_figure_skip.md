---
id: fixture_node_visitor_figure_skip
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
    visitFigureStart(ctx: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<p>See the chart below:</p><figure><img src=\"chart.svg\"><figcaption>Revenue Trends 2020-2024</figcaption></figure><p>As shown in the chart above.</p>", { visitor: _testVisitor as any });
}

void main();

```
