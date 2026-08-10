```typescript title="WebAssembly"
import { WasmConversionOptions, WasmVisitorHandle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const _testVisitor = {
    visitAudio(ctx: any, src: any): string | { Custom: string } {
        return { Custom: "[AUDIO: podcast.mp3]" };
    },

    }

  const result = convert("<p>Listen to this: <audio src=\"podcast.mp3\" controls></audio></p>", (() => { const _u = WasmConversionOptions.default(); _u.visitor = new WasmVisitorHandle(_testVisitor); return _u; })());
}

void main();

```
