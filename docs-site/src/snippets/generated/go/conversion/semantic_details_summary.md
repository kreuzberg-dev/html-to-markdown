---
id: fixture_go_semantic_details_summary
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
	result, err := htmd.Convert(`<details><summary>Click to expand</summary><p>Hidden content here.</p></details>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
