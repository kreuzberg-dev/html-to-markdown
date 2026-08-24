---
target: node
---

```typescript
import { convert } from "@xberg-io/html-to-markdown";

const html = `
<table>
    <tr><th>Name</th><th>Age</th></tr>
    <tr><td>Alice</td><td>30</td></tr>
    <tr><td>Bob</td><td>25</td></tr>
</table>
`;

const result = convert(html, { includeDocumentStructure: true });

for (const table of result.tables ?? []) {
  for (const cell of table.grid.cells ?? []) {
    const kind = cell.isHeader ? "Header" : "Cell";
    console.log(`  ${kind} (r${cell.row},c${cell.col}): ${cell.content}`);
  }
}
```
