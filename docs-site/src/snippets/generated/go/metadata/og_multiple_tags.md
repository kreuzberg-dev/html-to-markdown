---
id: fixture_go_og_multiple_tags
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
	result, err := htmd.Convert(`<html><head><meta property="og:title" content="Article Title"><meta property="og:type" content="article"><meta property="og:url" content="https://example.com/article"><meta property="og:site_name" content="Example Site"><meta property="og:description" content="An interesting article."><meta property="og:image" content="https://example.com/article.jpg"></head><body><article><p>Article content here.</p></article></body></html>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
