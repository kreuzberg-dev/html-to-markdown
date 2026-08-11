---
id: fixture_node_visitor_definition_list_custom
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const _testVisitor = {
    visitDefinitionTerm(ctx: any, text: any): string | { Custom: string } {
        return { Custom: `**${text}**` };
    },

    }

  const result = convert("<dl><dt>HTML</dt><dd>HyperText Markup Language</dd><dt>CSS</dt><dd>Cascading Style Sheets</dd></dl>", { visitor: _testVisitor as any });
}

void main();

```
