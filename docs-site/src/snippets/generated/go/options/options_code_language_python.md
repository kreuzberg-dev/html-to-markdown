```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func ptr[T any](value T) *T { return &value }
func main() {
	options := htmd.ConversionOptions{
		CodeLanguage: ptr(`python`),
	}
	result, err := htmd.Convert(`<pre><code>def hello(): pass</code></pre>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
