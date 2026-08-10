```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<a href=\"mailto:a@b.com\">a@b.com</a>", undefined);
}

void main();

```
