```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{
		IncludeDocumentStructure: true,
	}
	result, err := htmd.Convert(`<p>No tables here</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
