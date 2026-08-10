```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<p>This is <u>very important</u> text.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
