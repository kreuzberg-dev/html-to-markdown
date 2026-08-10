```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<table><thead><tr><th colspan="2">Full Name</th></tr></thead><tbody><tr><td>John</td><td>Doe</td></tr></tbody></table>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
