```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p>中文内容</p><p>日本語テキスト</p><p>한국어 텍스트</p>", undefined);
}

void main();

```
