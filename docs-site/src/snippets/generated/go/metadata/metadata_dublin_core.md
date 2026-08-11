---
id: fixture_go_metadata_dublin_core
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
	result, err := htmd.Convert(`<html><head><title>Scholarly Work</title><meta name="DC.title" content="Principles of Knowledge Management"><meta name="DC.creator" content="Dr. Alice Johnson"><meta name="DC.date" content="2023-06-15"><meta name="DC.subject" content="Knowledge Management"><meta name="DC.publisher" content="Academic Press"></head><body><p>This is a scholarly article.</p></body></html>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
