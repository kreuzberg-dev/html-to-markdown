```javascript
import { convert, WasmConversionOptions, WasmVisitorHandle } from "@xberg-io/html-to-markdown-wasm";

const visitor = {
  visitLink(ctx, href, text, title) {
    return { Custom: `[${text}](${href} "external")` };
  },
};

const options = WasmConversionOptions.default();
options.visitor = new WasmVisitorHandle(visitor);

const result = convert('<h1>Hello</h1><a href="https://example.com">link</a>', options);
console.log(result.content);
```
