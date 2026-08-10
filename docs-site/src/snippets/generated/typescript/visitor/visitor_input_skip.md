```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitInput(ctx: any, input_type: any, name: any, value: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<p>Sign up:</p><input type=\"text\" name=\"email\" placeholder=\"your@email.com\"><input type=\"checkbox\" name=\"agree\"><p>Continue</p>", { visitor: _testVisitor as any });
}

void main();

```
