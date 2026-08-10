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
	result, err := htmd.Convert(`<html><head><title>Company</title></head><body><div itemscope itemtype="https://schema.org/Organization"><span itemprop="name">Acme Corp</span><span itemprop="foundingDate">2020</span><span itemprop="url">https://acmecorp.example.com</span><span itemprop="logo">https://acmecorp.example.com/logo.png</span></div></body></html>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
