```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<blockquote><p>A wise quote.</p></blockquote>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
