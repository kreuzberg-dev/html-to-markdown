```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{
		ConvertAsInline: true,
	}
	result, err := htmd.Convert(`<p>One</p><p>Two</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
