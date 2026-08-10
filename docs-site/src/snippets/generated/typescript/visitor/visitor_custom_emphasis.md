```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitEmphasis(ctx: any, text: any): string | { Custom: string } {
        return { Custom: `>>>${text}<<<` };
    },

    }

  const result = convert("<p>This is <em>important</em> text.</p>", { visitor: _testVisitor as any });
}

void main();

```
