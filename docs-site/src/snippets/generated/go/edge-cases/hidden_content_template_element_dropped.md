---
id: fixture_go_hidden_content_template_element_dropped
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
	result, err := htmd.Convert(`<p>visible</p><template><p>secret template text</p></template><p>also visible</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
