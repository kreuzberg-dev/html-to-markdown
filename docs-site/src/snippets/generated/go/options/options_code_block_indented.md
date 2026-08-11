---
id: fixture_go_options_code_block_indented
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

func ptr[T any](value T) *T { return &value }
func main() {
	options := htmd.ConversionOptions{
		CodeBlockStyle: ptr(htmd.CodeBlockStyle(`Indented`)),
	}
	result, err := htmd.Convert(`<pre><code>print('hello')</code></pre>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
