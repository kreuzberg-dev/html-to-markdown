---
target: wasm
---

```javascript
import { convert } from "@xberg-io/html-to-markdown-wasm";

const html = "<h1>Hello</h1><p>This is <strong>fast</strong>!</p>";
const result = convert(html);
const markdown = result.content;
console.log(markdown);
```
