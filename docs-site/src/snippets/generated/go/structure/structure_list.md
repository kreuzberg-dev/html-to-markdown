---
id: fixture_go_structure_list
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
	result, err := htmd.Convert(`<p>Items:</p><ul><li>Alpha</li><li>Beta</li><li>Gamma</li></ul>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
