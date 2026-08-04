```typescript
import { convert } from "@xberg-io/html-to-markdown";

const result = convert("<h1>Hello World</h1>");
const markdown = result.content ?? "";
console.log(markdown); // # Hello World
```
