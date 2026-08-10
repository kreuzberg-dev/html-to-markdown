```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { preprocessing: { removeForms: false } };
  const result = convert("<form><label>Message:</label><textarea>Default text content</textarea></form>", options);
}

void main();

```
