---
id: fixture_go_image_figure_figcaption
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
	result, err := htmd.Convert(`<figure><img src="sunset.jpg" alt="A sunset"><figcaption>Beautiful sunset over the ocean</figcaption></figure>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
