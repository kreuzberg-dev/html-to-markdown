---
id: fixture_go_semantic_sub_superscript
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
	result, err := htmd.Convert(`<p>H<sub>2</sub>O and E=mc<sup>2</sup></p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
