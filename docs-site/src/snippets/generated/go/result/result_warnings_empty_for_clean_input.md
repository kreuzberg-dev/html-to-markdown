---
id: fixture_go_result_warnings_empty_for_clean_input
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
	result, err := htmd.Convert(`<h1>Title</h1><p>Clean content with <a href='https://example.com'>a link</a>.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
