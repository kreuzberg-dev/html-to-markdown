---
id: fixture_go_visitor_subscript_skip
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
	result, err := htmd.Convert(`<p>The formula C<sub>12</sub>H<sub>22</sub>O<sub>11</sub> is sugar.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
