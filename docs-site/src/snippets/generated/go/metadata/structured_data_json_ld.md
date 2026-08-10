```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<html><head><title>Article</title><script type="application/ld+json">{"@context":"https://schema.org","@type":"Article","headline":"My Article","author":{"@type":"Person","name":"Jane Doe"},"datePublished":"2024-01-15"}</script></head><body><h1>My Article</h1><p>Article body text.</p></body></html>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
