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
	result, err := htmd.Convert(`<html><head><title>Contact</title></head><body><div itemscope itemtype="https://schema.org/Person"><span itemprop="name">John Smith</span><span itemprop="email">john@example.com</span><span itemprop="telephone">+1-555-0100</span></div></body></html>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
