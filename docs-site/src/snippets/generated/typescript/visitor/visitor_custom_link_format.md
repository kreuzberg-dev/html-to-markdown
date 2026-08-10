```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitLink(ctx: any, href: any, text: any, title: any): string | { Custom: string } {
        return { Custom: `${text} (${href})` };
    },

    }

  const result = convert("<p>Visit <a href=\"https://example.com\">Example</a> for more info.</p>", { visitor: _testVisitor as any });
}

void main();

```
