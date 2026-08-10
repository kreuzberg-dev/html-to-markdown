```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitButton(ctx: any, text: any): string | { Custom: string } {
        return { Custom: `[BTN:${text}]` };
    },

    }

  const result = convert("<p>Confirm action: <button type=\"submit\">Click me</button> or <button type=\"reset\">Cancel</button></p>", { visitor: _testVisitor as any });
}

void main();

```
