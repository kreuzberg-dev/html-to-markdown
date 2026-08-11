---
id: fixture_go_options_compact_tables_true
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
		CompactTables: true,
	}
	result, err := htmd.Convert(`<table><thead><tr><th>Name</th><th>Score</th></tr></thead><tbody><tr><td>Alice</td><td>100</td></tr><tr><td>Bob</td><td>42</td></tr></tbody></table>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
