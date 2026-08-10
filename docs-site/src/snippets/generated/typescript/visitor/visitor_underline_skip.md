```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitUnderline(ctx: any, text: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<p>Normal text with <u>underlined part</u> and more text.</p>", { visitor: _testVisitor as any });
}

void main();

```
