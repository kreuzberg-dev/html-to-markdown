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
	result, err := htmd.Convert(`<p><img src="https://example.com/photo.jpg" alt="A photo"></p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
