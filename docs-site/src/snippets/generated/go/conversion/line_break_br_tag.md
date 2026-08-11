---
id: fixture_go_line_break_br_tag
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
	result, err := htmd.Convert(`<p>First line.<br>Second line.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
