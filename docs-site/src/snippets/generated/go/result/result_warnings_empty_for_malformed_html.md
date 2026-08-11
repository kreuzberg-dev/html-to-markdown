---
id: fixture_go_result_warnings_empty_for_malformed_html
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
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<p>Unclosed paragraph<div>Mixed nesting</p></div>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
