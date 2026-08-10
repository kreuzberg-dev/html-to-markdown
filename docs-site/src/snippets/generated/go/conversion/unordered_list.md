```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
