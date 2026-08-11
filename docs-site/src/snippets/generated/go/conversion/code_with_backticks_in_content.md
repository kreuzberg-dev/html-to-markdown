---
id: fixture_go_code_with_backticks_in_content
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
	result, err := htmd.Convert("<p>Use <code>`backtick` here</code> carefully.</p>", options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
