---
target: node
---

```typescript
import { convert, ConversionOptions, HeadingStyle } from "@xberg-io/html-to-markdown";

const options: ConversionOptions = {
  headingStyle: HeadingStyle.Atx,
  listIndentWidth: 2,
  wrap: true,
};

const result = convert("<h1>Title</h1><p>Content</p>", options);
const markdown = result.content;
```
