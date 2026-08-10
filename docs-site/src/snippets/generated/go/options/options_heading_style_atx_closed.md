```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func ptr[T any](value T) *T { return &value }
func main() {
	options := htmd.ConversionOptions{
		HeadingStyle: ptr(htmd.HeadingStyle(`AtxClosed`)),
	}
	result, err := htmd.Convert(`<h1>Closed Heading</h1>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
