```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<ol><li>Step 1<ol><li>Step 1a</li><li>Step 1b</li></ol></li><li>Step 2</li></ol>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
