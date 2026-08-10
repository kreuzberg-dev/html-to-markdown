```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func ptr[T any](value T) *T { return &value }
func main() {
	options := htmd.ConversionOptions{
		SubSymbol: ptr(`~`),
	}
	result, err := htmd.Convert(`<p>H<sub>2</sub>O</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
