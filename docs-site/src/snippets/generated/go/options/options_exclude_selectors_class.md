---
id: fixture_go_options_exclude_selectors_class
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
	result, err := htmd.Convert(`<body><div class="cookie-banner">Accept cookies</div><p>Main content</p></body>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
