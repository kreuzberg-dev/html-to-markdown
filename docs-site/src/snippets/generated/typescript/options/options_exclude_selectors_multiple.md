```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { excludeSelectors: [".nav", "footer"] };
  const result = convert("<body><nav class=\"nav\">Menu</nav><p>Content</p><footer>Footer</footer></body>", options);
}

void main();

```
