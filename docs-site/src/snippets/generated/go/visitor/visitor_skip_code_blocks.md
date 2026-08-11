---
id: fixture_go_visitor_skip_code_blocks
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
	result, err := htmd.Convert(`<p>Intro text</p><pre><code>let x = 42;</code></pre><p>Outro text</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
