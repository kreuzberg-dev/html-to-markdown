---
id: fixture_go_emphasis_strikethrough_del
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
	result, err := htmd.Convert(`<p><del>deleted text</del></p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
