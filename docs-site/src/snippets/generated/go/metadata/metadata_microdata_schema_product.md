---
id: fixture_go_metadata_microdata_schema_product
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
	result, err := htmd.Convert(`<html><head><title>Product</title></head><body><div itemscope itemtype="https://schema.org/Product"><h1 itemprop="name">Awesome Widget</h1><span itemprop="description">The best widget on the market</span><span itemprop="price">29.99</span><span itemprop="priceCurrency">USD</span><img itemprop="image" src="widget.jpg" alt="Widget"><span itemprop="ratingValue">4.5</span></div></body></html>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
