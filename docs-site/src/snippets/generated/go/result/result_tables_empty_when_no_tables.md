---
id: fixture_go_result_tables_empty_when_no_tables
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
	result, err := htmd.Convert(`<p>No tables here</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
