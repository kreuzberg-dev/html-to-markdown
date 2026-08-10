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
	result, err := htmd.Convert(`<p>See <a href="https://example.com">Example</a> for details.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
