---
target: wasm
---

```javascript
import { convert, WasmConversionOptions } from "@xberg-io/html-to-markdown-wasm";

const html = `
<table>
    <tr><th>Name</th><th>Age</th></tr>
    <tr><td>Alice</td><td>30</td></tr>
    <tr><td>Bob</td><td>25</td></tr>
</table>
`;

const options = WasmConversionOptions.default();
options.includeDocumentStructure = true;

const result = convert(html, options);

for (const table of result.tables) {
  for (const cell of table.grid.cells) {
    const kind = cell.isHeader ? "Header" : "Cell";
    console.log(`  ${kind} (r${cell.row},c${cell.col}): ${cell.content}`);
  }
}
```
