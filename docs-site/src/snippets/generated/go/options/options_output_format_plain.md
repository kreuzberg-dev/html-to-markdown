---
id: fixture_go_options_output_format_plain
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
		OutputFormat: ptr(htmd.OutputFormat(`Plain`)),
	}
	result, err := htmd.Convert(`<h1>Title</h1><p>Some <strong>bold</strong> text.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
