---
id: fixture_go_options_include_document_structure_true
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
	result, err := htmd.Convert(`<article><h1>Heading</h1><p>Paragraph body.</p></article>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
