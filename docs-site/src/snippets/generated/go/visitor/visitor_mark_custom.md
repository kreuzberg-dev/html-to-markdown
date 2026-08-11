---
id: fixture_go_visitor_mark_custom
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
	result, err := htmd.Convert(`<p>This is a <mark>highlighted passage</mark> in the text.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
