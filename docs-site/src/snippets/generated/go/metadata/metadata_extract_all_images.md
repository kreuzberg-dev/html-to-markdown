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
	result, err := htmd.Convert(`<html><head><title>Gallery</title></head><body><img src="https://example.com/photo1.jpg" alt="Photo 1"><img src="https://example.com/photo2.png" alt="Photo 2"><img src="/local/image.webp" alt="Local image"></body></html>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
