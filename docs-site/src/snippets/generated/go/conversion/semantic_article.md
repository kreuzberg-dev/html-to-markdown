```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<article><h2>Article Title</h2><p>Article body.</p></article>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
