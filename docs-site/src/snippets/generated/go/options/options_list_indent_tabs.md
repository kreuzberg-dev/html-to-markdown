```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func ptr[T any](value T) *T { return &value }
func main() {
	options := htmd.ConversionOptions{
		ListIndentType: ptr(htmd.ListIndentType(`Tabs`)),
	}
	result, err := htmd.Convert(`<ul><li>Parent<ul><li>Child</li></ul></li></ul>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
