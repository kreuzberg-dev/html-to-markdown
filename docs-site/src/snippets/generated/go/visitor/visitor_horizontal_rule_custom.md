```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<h1>Section A</h1><p>Content A</p><hr><h1>Section B</h1><p>Content B</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
