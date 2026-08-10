```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<html><head><meta name="twitter:card" content="summary_large_image"><meta name="twitter:site" content="@examplesite"><meta name="twitter:title" content="Twitter Card Title"><meta name="twitter:description" content="Twitter card description."><meta name="twitter:image" content="https://example.com/twitter-image.jpg"></head><body><p>Content</p></body></html>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
