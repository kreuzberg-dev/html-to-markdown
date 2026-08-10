```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitAudio(ctx: any, src: any): string | { Custom: string } {
        return { Custom: "[AUDIO: podcast.mp3]" };
    },

    }

  const result = convert("<p>Listen to this: <audio src=\"podcast.mp3\" controls></audio></p>", { visitor: _testVisitor as any });
}

void main();

```
