---
id: fixture_go_ordered_list
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
	result, err := htmd.Convert(`<ol><li>First</li><li>Second</li><li>Third</li></ol>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
