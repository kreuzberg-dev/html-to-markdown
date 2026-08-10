```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func ptr[T any](value T) *T { return &value }
func main() {
	options := htmd.ConversionOptions{
		LinkStyle: ptr(htmd.LinkStyle(`Reference`)),
	}
	result, err := htmd.Convert(`<p><a href='https://example.com'>Example</a> and <a href='https://other.com'>Other</a></p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
