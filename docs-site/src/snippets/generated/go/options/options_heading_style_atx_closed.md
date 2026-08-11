---
id: fixture_go_options_heading_style_atx_closed
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
		HeadingStyle: ptr(htmd.HeadingStyle(`AtxClosed`)),
	}
	result, err := htmd.Convert(`<h1>Closed Heading</h1>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
