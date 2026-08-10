```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{
		EscapeASCII: true,
	}
	result, err := htmd.Convert(`<p>Text with # hash and [brackets] and * star</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
