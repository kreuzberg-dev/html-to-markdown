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
	result, err := htmd.Convert(`<a href="/file (1).pdf">file</a>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
