---
id: fixture_node_visitor_definition_list_skip
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
    visitDefinitionDescription(ctx: any, text: any): string | { Custom: string } {
        return "Skip";
    },

    visitDefinitionTerm(ctx: any, text: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<p>Glossary:</p><dl><dt>Term A</dt><dd>Definition of term A</dd><dt>Term B</dt><dd>Definition of term B</dd></dl><p>End of glossary</p>", { visitor: _testVisitor as any });
}

void main();

```
