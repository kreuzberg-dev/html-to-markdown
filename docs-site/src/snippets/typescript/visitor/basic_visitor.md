---
target: node
---

```typescript
import { convert, ConversionOptions, NodeContext, VisitResult } from "@xberg-io/html-to-markdown";

// `visitor` is a plain object of camelCase callbacks; there is no exported
// `Visitor` type to annotate it with. Return `VisitResult.Continue` / `Skip` /
// `PreserveHtml` for the built-in behaviors, or `{ Custom: "..." }` to replace
// the node's output with custom markdown.
const visitor = {
  visitLink(ctx: NodeContext, href: string, text: string) {
    // Custom handling for links
    return { Custom: `[${text}](${href})` };
  },
  visitHeading(ctx: NodeContext, level: number, text: string): VisitResult {
    // Fall back to the default handling for headings
    return VisitResult.Continue;
  },
};

const options: ConversionOptions = { visitor };
const result = convert('<h1>Title</h1><a href="url">Link</a>', options);
const markdown = result.content;
```
