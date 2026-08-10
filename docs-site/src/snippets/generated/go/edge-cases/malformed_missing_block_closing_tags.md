```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<div><h1>Title<p>First paragraph<p>Second paragraph</div>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
