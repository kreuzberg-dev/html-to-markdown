---
id: fixture_go_options_strip_tags_div_span
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
	result, err := htmd.Convert(`<div class='wrapper'><p>Inside div</p></div><p>Outside <span class='hl'>span text</span></p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
