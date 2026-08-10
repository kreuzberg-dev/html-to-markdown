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
	result, err := htmd.Convert(`<h1>Main Title</h1><h2>Section One</h2><p>Section one content.</p><h2>Section Two</h2><p>Section two content.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
