```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { excludeSelectors: ["[role='complementary']"] };
  const result = convert("<body><div role=\"complementary\">Sidebar</div><p>Primary text</p></body>", options);
}

void main();

```
