---
id: fixture_go_options_max_depth_truncates
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
		MaxDepth: 3,
	}
	result, err := htmd.Convert(`<div><p>Shallow</p><div><div><div><p>Too deep</p></div></div></div></div>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
