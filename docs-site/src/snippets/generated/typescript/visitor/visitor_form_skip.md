```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitForm(ctx: any, actionUrl: any, method: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<p>Before form</p><form><input type=\"email\" name=\"email\"></form><p>After form</p>", { visitor: _testVisitor as any });
}

void main();

```
