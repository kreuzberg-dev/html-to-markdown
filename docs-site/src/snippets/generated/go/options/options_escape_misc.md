---
id: fixture_go_options_escape_misc
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
		EscapeMisc: true,
	}
	result, err := htmd.Convert(`<p>Use # and | and ~ in text.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
