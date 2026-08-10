```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { skipImages: true };
  const result = convert("<p>Before <img src='test.jpg' alt='photo'> After</p>", options);
}

void main();

```
