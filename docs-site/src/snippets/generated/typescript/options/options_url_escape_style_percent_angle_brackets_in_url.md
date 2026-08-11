---
id: fixture_node_options_url_escape_style_percent_angle_brackets_in_url
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
  const result = convert("<a href=\"/file (1) <draft>.pdf\">file</a>", options);
}

void main();

```
