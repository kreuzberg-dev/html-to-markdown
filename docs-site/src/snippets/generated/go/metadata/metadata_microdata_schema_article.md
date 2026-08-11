---
id: fixture_go_metadata_microdata_schema_article
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
	result, err := htmd.Convert(`<html><head><title>Article</title></head><body><article itemscope itemtype="https://schema.org/Article"><h1 itemprop="headline">Breaking News Today</h1><span itemprop="author">Jane Reporter</span><span itemprop="datePublished">2024-04-22</span><div itemprop="articleBody"><p>The article content goes here with important information about the breaking news story.</p></div></article></body></html>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
