---
id: fixture_go_visitor_input_skip
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
	result, err := htmd.Convert(`<p>Sign up:</p><input type="text" name="email" placeholder="your@email.com"><input type="checkbox" name="agree"><p>Continue</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
