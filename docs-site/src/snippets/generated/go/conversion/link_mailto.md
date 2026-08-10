```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<a href="mailto:user@example.com">Email us</a>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
