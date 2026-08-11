---
id: fixture_go_visitor_horizontal_rule_skip
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
	result, err := htmd.Convert(`<p>Part 1</p><hr><p>Part 2</p><hr><p>Part 3</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
