```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{
		EscapeAsterisks: true,
	}
	result, err := htmd.Convert(`<p>Use 2*3 = 6 in math.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
