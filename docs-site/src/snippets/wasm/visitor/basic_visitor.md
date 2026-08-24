---
target: wasm
---

```typescript
import { convert, WasmConversionOptions, WasmNodeContext, WasmVisitorHandle } from "@xberg-io/html-to-markdown-wasm";

const visitor = {
  visitLink(ctx: WasmNodeContext, href: string, text: string, title: string) {
    return { Custom: `[${text}](${href} "external")` };
  },
};

const options = WasmConversionOptions.default();
options.visitor = new WasmVisitorHandle(visitor);

const result = convert('<h1>Hello</h1><a href="https://example.com">link</a>', options);
console.log(result.content);
```
