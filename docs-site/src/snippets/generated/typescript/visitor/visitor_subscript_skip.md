```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitSubscript(ctx: any, text: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<p>The formula C<sub>12</sub>H<sub>22</sub>O<sub>11</sub> is sugar.</p>", { visitor: _testVisitor as any });
}

void main();

```
