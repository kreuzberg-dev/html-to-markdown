---
id: fixture_go_empty_html
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
	result, err := htmd.Convert(`<html><head></head><body></body></html>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
