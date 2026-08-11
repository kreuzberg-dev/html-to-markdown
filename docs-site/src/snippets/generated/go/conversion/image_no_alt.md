---
id: fixture_go_image_no_alt
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
	result, err := htmd.Convert(`<img src="banner.jpg">`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
