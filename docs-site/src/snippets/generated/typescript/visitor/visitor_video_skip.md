```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitVideo(ctx: any, src: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<h2>Demo</h2><video src=\"demo.webm\"></video><p>See the demo above.</p>", { visitor: _testVisitor as any });
}

void main();

```
