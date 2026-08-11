---
id: fixture_go_code_block_no_language
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
	result, err := htmd.Convert(`<pre><code>plain code here</code></pre>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
