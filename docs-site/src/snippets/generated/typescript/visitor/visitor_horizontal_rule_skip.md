```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitHorizontalRule(ctx: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<p>Part 1</p><hr><p>Part 2</p><hr><p>Part 3</p>", { visitor: _testVisitor as any });
}

void main();

```
