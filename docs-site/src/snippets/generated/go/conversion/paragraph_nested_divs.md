---
id: fixture_go_paragraph_nested_divs
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
	result, err := htmd.Convert(`<div><div><p>Nested text</p></div></div>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
