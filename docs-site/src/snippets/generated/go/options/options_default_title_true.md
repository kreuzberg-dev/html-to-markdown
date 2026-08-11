---
id: fixture_go_options_default_title_true
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
		DefaultTitle: true,
	}
	result, err := htmd.Convert(`<p><a href='https://example.com'>Link</a></p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
