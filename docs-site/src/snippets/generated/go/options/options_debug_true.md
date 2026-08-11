---
id: fixture_go_options_debug_true
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
	options := htmd.ConversionOptions{
		Debug: true,
	}
	result, err := htmd.Convert(`<p>Debug test</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
