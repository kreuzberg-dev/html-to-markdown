```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<blockquote><p>Quote intro:</p><ul><li>Point one</li><li>Point two</li></ul></blockquote>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
