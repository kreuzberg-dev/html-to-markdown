```typescript title="TypeScript"
import { ConversionOptions, PreprocessingPreset, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { preprocessing: { preset: PreprocessingPreset.Minimal } };
  const result = convert("<nav>Navigation</nav><p>Content</p><footer>Footer</footer>", options);
}

void main();

```
