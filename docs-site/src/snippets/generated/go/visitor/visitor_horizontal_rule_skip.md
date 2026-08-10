```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<p>Part 1</p><hr><p>Part 2</p><hr><p>Part 3</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
