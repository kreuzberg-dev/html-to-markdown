---
id: fixture_go_xss_onclick_handler_removed
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
	result, err := htmd.Convert(`<p><a href="https://example.com" onclick="alert('xss')">Click me</a></p><button onmouseover="steal_data()">Hover me</button>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
