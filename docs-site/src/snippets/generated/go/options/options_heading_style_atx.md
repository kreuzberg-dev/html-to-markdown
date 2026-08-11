---
id: fixture_go_options_heading_style_atx
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
		HeadingStyle: ptr(htmd.HeadingStyle(`Atx`)),
	}
	result, err := htmd.Convert(`<h1>Title</h1><h2>Subtitle</h2>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
