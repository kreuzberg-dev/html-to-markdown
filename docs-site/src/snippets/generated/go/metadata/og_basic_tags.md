```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<html><head><title>Fallback Title</title><meta property="og:title" content="OG Title"><meta property="og:description" content="OG description text."><meta property="og:image" content="https://example.com/image.jpg"></head><body><p>Content</p></body></html>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
