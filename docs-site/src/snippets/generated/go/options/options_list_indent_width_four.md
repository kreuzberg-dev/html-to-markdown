---
id: fixture_go_options_list_indent_width_four
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
	options := htmd.ConversionOptions{
		ListIndentWidth: 4,
	}
	result, err := htmd.Convert(`<ul><li>Outer<ul><li>Inner</li></ul></li></ul>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
