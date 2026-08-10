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
	result, err := htmd.Convert(`<h1>Top Level</h1><p>Top intro.</p><h2>Mid Level</h2><p>Mid content.</p><h3>Deep Level</h3><p>Deep content.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
