```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitVideo(ctx: any, src: any): string | { Custom: string } {
        return { Custom: `[VIDEO: ${src}]` };
    },

    }

  const result = convert("<p>Watch our tutorial:</p><video src=\"tutorial.mp4\" width=\"320\" height=\"240\" controls></video><p>Great content!</p>", { visitor: _testVisitor as any });
}

void main();

```
