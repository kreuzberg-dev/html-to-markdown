```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func ptr[T any](value T) *T { return &value }
func main() {
	options := htmd.ConversionOptions{
		HeadingStyle: ptr(htmd.HeadingStyle(`Atx`)),
		OutputFormat: ptr(htmd.OutputFormat(`Markdown`)),
	}
	result, err := htmd.Convert(`<h1>Title</h1><p>Some text.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
