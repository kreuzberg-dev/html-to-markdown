```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func ptr[T any](value T) *T { return &value }
func main() {
	options := htmd.ConversionOptions{
		URLEscapeStyle: ptr(htmd.URLEscapeStyle(`percent`)),
	}
	result, err := htmd.Convert(`<img src="/img (1) <draft>.png" alt="alt">`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
