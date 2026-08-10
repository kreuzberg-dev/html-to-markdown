```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitLineBreak(ctx: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<p>Address Line 1<br>Address Line 2<br>Address Line 3</p>", { visitor: _testVisitor as any });
}

void main();

```
