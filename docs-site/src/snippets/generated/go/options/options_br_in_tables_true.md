```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{
		BrInTables: true,
	}
	result, err := htmd.Convert(`<table><tr><th>Header</th></tr><tr><td>Line 1<br>Line 2</td></tr></table>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
