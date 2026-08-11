---
id: fixture_go_options_newline_backslash
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
		NewlineStyle: ptr(htmd.NewlineStyle(`Backslash`)),
	}
	result, err := htmd.Convert(`<p>Line one<br>Line two</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
