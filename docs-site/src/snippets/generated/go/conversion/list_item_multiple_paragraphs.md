```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<ul><li><p>First paragraph in item.</p><p>Second paragraph in item.</p></li><li>Simple item</li></ul>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
