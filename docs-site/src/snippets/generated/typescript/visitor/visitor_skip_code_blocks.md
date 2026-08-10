```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitCodeBlock(ctx: any, lang: any, code: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<p>Intro text</p><pre><code>let x = 42;</code></pre><p>Outro text</p>", { visitor: _testVisitor as any });
}

void main();

```
