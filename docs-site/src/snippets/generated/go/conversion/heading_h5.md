---
id: fixture_go_heading_h5
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
	result, err := htmd.Convert(`<h5>Heading 5</h5>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
