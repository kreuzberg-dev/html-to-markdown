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
	result, err := htmd.Convert(`<html lang="ar" dir="rtl"><head><title>RTL Document</title></head><body><p>This is right-to-left text.</p></body></html>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
