```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func ptr[T any](value T) *T { return &value }
func main() {
	options := htmd.ConversionOptions{
		OutputFormat: ptr(htmd.OutputFormat(`Plain`)),
	}
	result, err := htmd.Convert(`<body><div class="nav">Navigation</div><p>Article body</p></body>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
