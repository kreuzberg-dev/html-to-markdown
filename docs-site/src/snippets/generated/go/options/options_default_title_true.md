```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{
		DefaultTitle: true,
	}
	result, err := htmd.Convert(`<p><a href='https://example.com'>Link</a></p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
