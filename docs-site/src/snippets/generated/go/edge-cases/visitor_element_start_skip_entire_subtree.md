---
id: fixture_go_visitor_element_start_skip_entire_subtree
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
	result, err := htmd.Convert(`<div><h1>Title</h1><p>Content</p></div>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
