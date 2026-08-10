```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitSuperscript(ctx: any, text: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<p>The equation x<sup>3</sup> + y<sup>3</sup> = z<sup>3</sup> has no solutions.</p>", { visitor: _testVisitor as any });
}

void main();

```
