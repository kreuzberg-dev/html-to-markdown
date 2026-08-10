```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{
		ExtractMetadata: true,
	}
	result, err := htmd.Convert(`<p>Contact <a href="mailto:hello@example.com">us</a> directly.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
