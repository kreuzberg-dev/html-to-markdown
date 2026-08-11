---
id: fixture_go_heading_h1
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
	result, err := htmd.Convert(`<h1>Heading 1</h1>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
