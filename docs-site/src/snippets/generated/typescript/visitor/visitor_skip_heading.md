```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitHeading(ctx: any, level: any, text: any, id: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<h1>Title</h1><p>Body text remains.</p>", { visitor: _testVisitor as any });
}

void main();

```
