---
id: fixture_go_structure_h1_h2_nested_group
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
	result, err := htmd.Convert(`<h1>Chapter One</h1><p>Chapter intro.</p><h2>Section One</h2><p>Section content.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
