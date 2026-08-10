```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { extractImages: false };
  const result = convert("<p>Text with <img src=\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==\" alt=\"pixel\"> image.</p>", options);
}

void main();

```
