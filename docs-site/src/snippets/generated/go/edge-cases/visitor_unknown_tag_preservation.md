---
id: fixture_go_visitor_unknown_tag_preservation
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
	result, err := htmd.Convert(`<article><p>Article text</p><x-custom>Custom element with content</x-custom><p>More article text</p></article>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
