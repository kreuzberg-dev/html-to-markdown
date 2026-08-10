```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<div><custom-tag>Custom content</custom-tag></div>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
