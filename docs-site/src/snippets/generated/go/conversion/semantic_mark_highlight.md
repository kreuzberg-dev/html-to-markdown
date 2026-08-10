```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<p>This is <mark>highlighted text</mark> in a sentence.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
