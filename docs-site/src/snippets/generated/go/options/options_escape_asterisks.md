---
id: fixture_go_options_escape_asterisks
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
		EscapeAsterisks: true,
	}
	result, err := htmd.Convert(`<p>Use 2*3 = 6 in math.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
