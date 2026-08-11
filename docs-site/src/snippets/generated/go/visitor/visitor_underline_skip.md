---
id: fixture_go_visitor_underline_skip
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
	result, err := htmd.Convert(`<p>Normal text with <u>underlined part</u> and more text.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
