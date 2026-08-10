```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitLineBreak(ctx: any): string | { Custom: string } {
        return { Custom: " | " };
    },

    }

  const result = convert("<p>First line<br>Second line<br>Third line</p>", { visitor: _testVisitor as any });
}

void main();

```
