```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { preprocessing: { enabled: false } };
  const result = convert("<nav>NavSection</nav><p>Paragraph</p>", options);
}

void main();

```
