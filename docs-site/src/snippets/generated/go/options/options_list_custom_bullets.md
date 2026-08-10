```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func ptr[T any](value T) *T { return &value }
func main() {
	options := htmd.ConversionOptions{
		Bullets: ptr(`*`),
	}
	result, err := htmd.Convert(`<ul><li>Item A</li><li>Item B</li></ul>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
