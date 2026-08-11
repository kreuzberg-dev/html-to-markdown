---
id: fixture_go_conversion_autolink_https_url
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
	result, err := htmd.Convert(`<a href="https://example.com">https://example.com</a>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
