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
	result, err := htmd.Convert(`<html lang="en" dir="ltr"><head><title>LTR Document</title></head><body><p>This is left-to-right text.</p></body></html>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
