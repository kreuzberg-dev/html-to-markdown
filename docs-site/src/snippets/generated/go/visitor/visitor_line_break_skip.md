---
id: fixture_go_visitor_line_break_skip
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
	result, err := htmd.Convert(`<p>Address Line 1<br>Address Line 2<br>Address Line 3</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
