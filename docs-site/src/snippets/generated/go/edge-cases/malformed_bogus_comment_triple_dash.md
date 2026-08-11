---
id: fixture_go_malformed_bogus_comment_triple_dash
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
	result, err := htmd.Convert(`<h1>One</h1>
<!-- /// --->
<p>Two</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
