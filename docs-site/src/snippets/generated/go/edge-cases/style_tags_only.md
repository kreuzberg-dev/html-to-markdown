```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<html><head><style>body { color: red; }</style></head><body><style>.foo { margin: 0; }</style></body></html>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
