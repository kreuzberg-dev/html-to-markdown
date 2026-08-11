---
id: fixture_go_options_skip_images_true
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
		SkipImages: true,
	}
	result, err := htmd.Convert(`<p>Before <img src='test.jpg' alt='photo'> After</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
