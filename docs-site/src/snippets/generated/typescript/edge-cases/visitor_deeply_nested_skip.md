```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitMark(ctx: any, text: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<div><p>Outer <em>emphasis <strong>with bold <mark>and highlight</mark></strong></em> text</p></div>", { visitor: _testVisitor as any });
}

void main();

```
