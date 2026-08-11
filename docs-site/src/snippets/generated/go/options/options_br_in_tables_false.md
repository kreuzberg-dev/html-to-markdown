---
id: fixture_go_options_br_in_tables_false
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
		BrInTables: false,
	}
	result, err := htmd.Convert(`<table><tr><th>Col</th></tr><tr><td>A<br>B</td></tr></table>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
