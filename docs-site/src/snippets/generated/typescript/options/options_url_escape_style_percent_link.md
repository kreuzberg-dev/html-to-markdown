```typescript title="TypeScript"
import { ConversionOptions, UrlEscapeStyle, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { urlEscapeStyle: UrlEscapeStyle.Percent };
  const result = convert("<a href=\"/file (1).pdf\">file</a>", options);
}

void main();

```
