```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{
		MaxDepth: 0,
	}
	result, err := htmd.Convert(`<p>Hello</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
