---
id: fixture_go_options_wrap_enabled
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
		Wrap:      true,
		WrapWidth: 40,
	}
	result, err := htmd.Convert(`<p>This is a long paragraph that should be wrapped at the specified column width when the wrap option is enabled.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
