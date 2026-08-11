---
id: fixture_go_heading_h2
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
	result, err := htmd.Convert(`<h2>Heading 2</h2>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
