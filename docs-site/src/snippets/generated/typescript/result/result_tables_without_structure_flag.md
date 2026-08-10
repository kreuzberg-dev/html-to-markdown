```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<table><tr><th>X</th></tr><tr><td>Y</td></tr></table>", undefined);
}

void main();

```
