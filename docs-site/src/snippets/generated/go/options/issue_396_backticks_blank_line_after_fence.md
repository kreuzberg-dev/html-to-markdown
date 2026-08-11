---
id: fixture_go_issue_396_backticks_blank_line_after_fence
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
		CodeBlockStyle: ptr(htmd.CodeBlockStyle(`Backticks`)),
	}
	result, err := htmd.Convert(`<p>Foo</p><pre><code>1
2
</code></pre><p>Bar</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
