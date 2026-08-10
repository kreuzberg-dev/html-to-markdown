```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitElementStart(ctx: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<div><h1>Title</h1><p>Content</p></div>", { visitor: _testVisitor as any });
}

void main();

```
