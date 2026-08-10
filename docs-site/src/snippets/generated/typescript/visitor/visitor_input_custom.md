```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitInput(ctx: any, input_type: any, name: any, value: any): string | { Custom: string } {
        return { Custom: `[INPUT:${input_type}]` };
    },

    }

  const result = convert("<form><label>Username: <input type=\"text\" name=\"username\" value=\"\"></label><label>Password: <input type=\"password\" name=\"password\"></label></form>", { visitor: _testVisitor as any });
}

void main();

```
