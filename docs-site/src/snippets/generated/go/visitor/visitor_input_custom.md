---
id: fixture_go_visitor_input_custom
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
	result, err := htmd.Convert(`<form><label>Username: <input type="text" name="username" value=""></label><label>Password: <input type="password" name="password"></label></form>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
