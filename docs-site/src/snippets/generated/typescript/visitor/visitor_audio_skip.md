```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitAudio(ctx: any, src: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<p>Background music:</p><audio src=\"music.ogg\" autoplay></audio><p>Enjoy!</p>", { visitor: _testVisitor as any });
}

void main();

```
