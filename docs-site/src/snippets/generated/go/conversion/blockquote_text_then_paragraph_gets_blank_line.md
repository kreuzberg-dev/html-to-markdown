---
id: fixture_go_blockquote_text_then_paragraph_gets_blank_line
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
	result, err := htmd.Convert(`<blockquote>Just text, then <p>a paragraph</p></blockquote>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
