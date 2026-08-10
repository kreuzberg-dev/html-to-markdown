```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitSuperscript(ctx: any, text: any): string | { Custom: string } {
        return { Custom: `^${text}^` };
    },

    }

  const result = convert("<p>Einstein's E=mc<sup>2</sup> revolutionized physics.</p>", { visitor: _testVisitor as any });
}

void main();

```
