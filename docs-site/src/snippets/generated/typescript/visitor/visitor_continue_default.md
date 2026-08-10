```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitStrong(ctx: any, text: any): string | { Custom: string } {
        return "Continue";
    },

    }

  const result = convert("<p>Hello <strong>World</strong></p>", { visitor: _testVisitor as any });
}

void main();

```
