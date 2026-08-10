```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func ptr[T any](value T) *T { return &value }
func main() {
	options := htmd.ConversionOptions{
		HighlightStyle: ptr(htmd.HighlightStyle(`DoubleEqual`)),
	}
	result, err := htmd.Convert(`<p>Text with <mark>highlighted</mark> here.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
