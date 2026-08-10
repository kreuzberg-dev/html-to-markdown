```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitImage(ctx: any, src: any, alt: any, title: any): string | { Custom: string } {
        return `[image: ${alt} -> ${src}]`;
    },

    }

  const result = convert("<img src=\"PhotoOne.JPG\" alt=\"Sunset Over Bay\">", { visitor: _testVisitor as any });
}

void main();

```
