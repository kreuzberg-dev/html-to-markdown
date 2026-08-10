```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<p>Einstein's E=mc<sup>2</sup> revolutionized physics.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
