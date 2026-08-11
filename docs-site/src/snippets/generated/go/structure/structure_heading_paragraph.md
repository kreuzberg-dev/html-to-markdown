---
id: fixture_go_structure_heading_paragraph
language: go
target: go
level: typecheck
requires: []
side_effect: safe
---

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
	result, err := htmd.Convert(`<h1>Title</h1><p>A paragraph of text.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
