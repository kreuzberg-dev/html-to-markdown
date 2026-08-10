```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<p>This has <strong>bold</strong>, <em>italic</em>, and a <a href="https://example.com">link</a>.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
