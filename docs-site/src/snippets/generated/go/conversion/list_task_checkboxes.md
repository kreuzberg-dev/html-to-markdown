---
id: fixture_go_list_task_checkboxes
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
	result, err := htmd.Convert(`<ul><li><input type="checkbox" checked> Done task</li><li><input type="checkbox"> Pending task</li></ul>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
