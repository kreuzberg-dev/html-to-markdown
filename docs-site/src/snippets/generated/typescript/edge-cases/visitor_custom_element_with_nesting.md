```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitCustomElement(ctx: any, tagName: any, html: any): string | { Custom: string } {
        return { Custom: "[CUSTOM WIDGET]" };
    },

    }

  const result = convert("<div><custom-widget data-value=\"123\"><p>Widget content here</p><span>With nested elements</span></custom-widget></div>", { visitor: _testVisitor as any });
}

void main();

```
