---
id: fixture_go_options_strip_newlines
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
		StripNewlines: true,
	}
	result, err := htmd.Convert(`<p>First paragraph.</p><p>Second paragraph.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
