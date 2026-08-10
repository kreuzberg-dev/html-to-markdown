```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { preprocessing: { removeNavigation: false } };
  const result = convert("<nav>SiteMenu</nav><main><p>MainContent</p></main><aside>SidebarText</aside>", options);
}

void main();

```
