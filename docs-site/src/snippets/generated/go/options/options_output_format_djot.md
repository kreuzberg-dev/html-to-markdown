```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func ptr[T any](value T) *T { return &value }
func main() {
	options := htmd.ConversionOptions{
		OutputFormat: ptr(htmd.OutputFormat(`Djot`)),
	}
	result, err := htmd.Convert(`<p>Simple paragraph.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
