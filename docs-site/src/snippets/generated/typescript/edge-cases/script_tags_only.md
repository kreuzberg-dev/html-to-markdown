```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<html><head><script>alert('xss')</script></head><body><script>document.write('hello')</script></body></html>", undefined);
}

void main();

```
