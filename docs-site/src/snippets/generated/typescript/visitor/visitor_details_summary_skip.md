```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitDetails(ctx: any, isOpen: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<p>Main content here.</p><details><summary>Hidden section</summary><p>Secret details</p></details><p>More main content.</p>", { visitor: _testVisitor as any });
}

void main();

```
