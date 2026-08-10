```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitMark(ctx: any, text: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<p>Key insight: <mark>always validate input</mark> for security.</p>", { visitor: _testVisitor as any });
}

void main();

```
