---
id: fixture_go_options_exclude_selectors_nested_content_dropped
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
	result, err := htmd.Convert(`<body><aside class="sidebar"><h2>Related</h2><p>Sidebar text</p></aside><main><p>Main text</p></main></body>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
