---
id: fixture_node_visitor_definition_list_custom_format
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
        return { Custom: `> ${text}` };
    },

    visitDefinitionTerm(ctx: any, text: any): string | { Custom: string } {
        return { Custom: `### ${text}` };
    },

    }

  const result = convert("<dl><dt>Python</dt><dd>A high-level programming language</dd><dt>JavaScript</dt><dd>A scripting language for web browsers</dd></dl>", { visitor: _testVisitor as any });
}

void main();

```
