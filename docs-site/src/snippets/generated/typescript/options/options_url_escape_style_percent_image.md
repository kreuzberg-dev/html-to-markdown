```typescript title="TypeScript"
import { ConversionOptions, UrlEscapeStyle, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { urlEscapeStyle: UrlEscapeStyle.Percent };
  const result = convert("<img src=\"/img (1) <draft>.png\" alt=\"alt\">", options);
}

void main();

```
