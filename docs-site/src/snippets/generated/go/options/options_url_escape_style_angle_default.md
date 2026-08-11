---
id: fixture_go_options_url_escape_style_angle_default
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
		URLEscapeStyle: ptr(htmd.URLEscapeStyle(`angle`)),
	}
	result, err := htmd.Convert(`<a href="/file (1).pdf">file</a>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
