---
id: fixture_go_visitor_form_custom
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
	result, err := htmd.Convert(`<div><form action="/submit" method="POST"><label>Name: <input type="text" name="name"></label><button type="submit">Submit</button></form></div>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
