---
id: fixture_go_semantic_section_with_heading
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
	result, err := htmd.Convert(`<section><h3>Section Heading</h3><p>Section content.</p></section>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
