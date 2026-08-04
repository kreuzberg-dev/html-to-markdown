```javascript
import { convert, WasmConversionOptions, WasmHeadingStyle } from "@xberg-io/html-to-markdown-wasm";

const options = WasmConversionOptions.default();
options.headingStyle = WasmHeadingStyle.Atx;
options.skipImages = true;

const result = convert('<h1>Hello</h1><img src="pic.jpg">', options);
const markdown = result.content;
console.log(markdown);
```
