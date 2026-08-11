---
id: fixture_go_options_exclude_selectors_id
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
	result, err := htmd.Convert(`<body><div id="ad-container">Buy stuff</div><p>Article text</p></body>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
