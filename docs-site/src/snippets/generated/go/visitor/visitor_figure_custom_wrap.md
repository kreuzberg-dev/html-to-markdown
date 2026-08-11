---
id: fixture_go_visitor_figure_custom_wrap
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
	result, err := htmd.Convert(`<section><h2>Gallery</h2><figure><img src="photo1.jpg" alt="Photo"><figcaption>Beautiful sunset</figcaption></figure></section>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
