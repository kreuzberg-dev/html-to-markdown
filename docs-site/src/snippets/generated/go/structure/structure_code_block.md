```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{
		IncludeDocumentStructure: true,
	}
	result, err := htmd.Convert(`<p>Example code:</p><pre><code class="language-rust">fn main() { println!("Hello"); }</code></pre>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
