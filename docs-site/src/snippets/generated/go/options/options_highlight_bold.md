---
id: fixture_go_options_highlight_bold
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

func ptr[T any](value T) *T { return &value }
func main() {
	options := htmd.ConversionOptions{
		HighlightStyle: ptr(htmd.HighlightStyle(`Bold`)),
	}
	result, err := htmd.Convert(`<p>Text with <mark>highlighted</mark> text.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
