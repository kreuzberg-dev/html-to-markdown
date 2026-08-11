---
id: fixture_go_metadata_link_type_anchor_classified
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
		ExtractMetadata: true,
	}
	result, err := htmd.Convert(`<p>Jump to <a href="#section">section</a> below.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
