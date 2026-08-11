---
id: fixture_go_metadata_extract_all_links
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
		ExtractMetadata: true,
	}
	result, err := htmd.Convert(`<html><head><title>Links Page</title></head><body><p>Visit <a href="https://example.com">Example</a> or <a href="https://docs.example.com">Docs</a>.</p><p>Also see <a href="/relative/path">relative link</a> and <a href="mailto:hello@example.com">email us</a>.</p></body></html>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
