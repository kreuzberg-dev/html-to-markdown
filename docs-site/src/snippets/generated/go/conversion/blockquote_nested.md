---
id: fixture_go_blockquote_nested
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
	result, err := htmd.Convert(`<blockquote><p>Outer quote.</p><blockquote><p>Inner quote.</p></blockquote></blockquote>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
