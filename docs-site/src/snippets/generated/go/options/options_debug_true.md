```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{
		Debug: true,
	}
	result, err := htmd.Convert(`<p>Debug test</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
