---
id: fixture_go_options_keep_inline_images_in_paragraph
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
	result, err := htmd.Convert(`<p>Text <img src='icon.png' alt='icon'> more text</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
