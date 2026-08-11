---
id: fixture_node_options_keep_inline_images_in_paragraph
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { keepInlineImagesIn: ["p"] };
  const result = convert("<p>Text <img src='icon.png' alt='icon'> more text</p>", options);
}

void main();

```
