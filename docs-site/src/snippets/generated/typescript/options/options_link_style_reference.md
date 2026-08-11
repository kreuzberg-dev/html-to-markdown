---
id: fixture_node_options_link_style_reference
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, LinkStyle, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { linkStyle: LinkStyle.Reference };
  const result = convert("<p><a href='https://example.com'>Example</a> and <a href='https://other.com'>Other</a></p>", options);
}

void main();

```
