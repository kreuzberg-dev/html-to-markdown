---
id: fixture_go_options_sup_symbol_caret
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
		SupSymbol: ptr(`^`),
	}
	result, err := htmd.Convert(`<p>x<sup>2</sup></p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
