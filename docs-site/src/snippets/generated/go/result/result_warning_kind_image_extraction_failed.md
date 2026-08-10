```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{
		ExtractImages: true,
	}
	result, err := htmd.Convert(`<p>Text<img src="data:BADMIME" alt="broken">end</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
