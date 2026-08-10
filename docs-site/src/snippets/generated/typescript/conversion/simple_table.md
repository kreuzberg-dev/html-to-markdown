```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<table><thead><tr><th>Name</th><th>Age</th></tr></thead><tbody><tr><td>Alice</td><td>30</td></tr></tbody></table>", undefined);
}

void main();

```
