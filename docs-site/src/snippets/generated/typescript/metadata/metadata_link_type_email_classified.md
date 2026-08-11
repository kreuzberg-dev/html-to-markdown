---
id: fixture_node_metadata_link_type_email_classified
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { extractMetadata: true };
  const result = convert("<p>Contact <a href=\"mailto:hello@example.com\">us</a> directly.</p>", options);
}

void main();

```
