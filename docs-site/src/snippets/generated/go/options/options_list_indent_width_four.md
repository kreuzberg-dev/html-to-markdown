```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{
		ListIndentWidth: 4,
	}
	result, err := htmd.Convert(`<ul><li>Outer<ul><li>Inner</li></ul></li></ul>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
