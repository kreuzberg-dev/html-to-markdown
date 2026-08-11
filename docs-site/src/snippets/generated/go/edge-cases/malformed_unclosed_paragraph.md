---
id: fixture_go_malformed_unclosed_paragraph
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
	result, err := htmd.Convert(`<p>This paragraph is never closed`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
