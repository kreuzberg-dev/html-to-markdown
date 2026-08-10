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
	result, err := htmd.Convert(`<html><head><title>Page</title><meta name="keywords" content="rust, markdown, html, converter"></head><body><p>Content</p></body></html>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
