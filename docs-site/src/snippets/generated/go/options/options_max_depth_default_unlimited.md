---
id: fixture_go_options_max_depth_default_unlimited
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
	result, err := htmd.Convert(`<div><div><div><div><p>Deep content</p></div></div></div></div>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
