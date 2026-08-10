```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<div><custom-widget data-value="123"><p>Widget content here</p><span>With nested elements</span></custom-widget></div>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
