---
id: fixture_go_visitor_skip_links
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
	result, err := htmd.Convert(`<p>Before <a href="https://example.com">link text</a> after</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
