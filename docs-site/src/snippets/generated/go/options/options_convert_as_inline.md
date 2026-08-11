---
id: fixture_go_options_convert_as_inline
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
		ConvertAsInline: true,
	}
	result, err := htmd.Convert(`<p>One</p><p>Two</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
