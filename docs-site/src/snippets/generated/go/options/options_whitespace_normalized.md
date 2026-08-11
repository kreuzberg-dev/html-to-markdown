---
id: fixture_go_options_whitespace_normalized
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
		WhitespaceMode: ptr(htmd.WhitespaceMode(`Normalized`)),
	}
	result, err := htmd.Convert(`<p>Text   with    extra   spaces.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
