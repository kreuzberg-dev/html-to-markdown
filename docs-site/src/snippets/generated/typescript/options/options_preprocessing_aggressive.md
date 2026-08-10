```typescript title="TypeScript"
import { ConversionOptions, PreprocessingPreset, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { preprocessing: { preset: PreprocessingPreset.Aggressive } };
  const result = convert("<nav>Menu</nav><article><h1>Title</h1><p>Content</p></article><aside>Sidebar</aside><footer>Footer</footer>", options);
}

void main();

```
