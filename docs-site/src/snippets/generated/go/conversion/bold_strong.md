---
id: fixture_go_bold_strong
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
	result, err := htmd.Convert(`<p><strong>bold</strong></p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
