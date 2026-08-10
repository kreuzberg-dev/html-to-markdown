```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitBlockquote(ctx: any, content: any, depth: any): string | { Custom: string } {
        return { Custom: `QUOTE: "${content}"` };
    },

    }

  const result = convert("<blockquote><p>A wise quote.</p></blockquote>", { visitor: _testVisitor as any });
}

void main();

```
