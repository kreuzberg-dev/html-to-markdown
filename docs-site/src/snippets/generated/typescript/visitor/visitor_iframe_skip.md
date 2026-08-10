```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitIframe(ctx: any, src: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<h3>Reviews</h3><iframe src=\"https://widget.example.com/reviews\"></iframe><p>See reviews from our partners.</p>", { visitor: _testVisitor as any });
}

void main();

```
