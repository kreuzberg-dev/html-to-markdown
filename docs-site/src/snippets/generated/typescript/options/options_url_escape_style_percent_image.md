---
id: fixture_node_options_url_escape_style_percent_image
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, UrlEscapeStyle, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { urlEscapeStyle: UrlEscapeStyle.Percent };
  const result = convert("<img src=\"/img (1) <draft>.png\" alt=\"alt\">", options);
}

void main();

```
