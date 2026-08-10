```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{
		Wrap: false,
	}
	result, err := htmd.Convert(`<p>This is a long paragraph that should not be wrapped at all because wrapping is disabled.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
