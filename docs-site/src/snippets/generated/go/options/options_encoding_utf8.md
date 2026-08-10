```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func ptr[T any](value T) *T { return &value }
func main() {
	options := htmd.ConversionOptions{
		Encoding: ptr(`utf-8`),
	}
	result, err := htmd.Convert(`<p>Café naïve résumé</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
