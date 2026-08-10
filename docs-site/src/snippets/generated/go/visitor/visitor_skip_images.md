```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<p>Before image</p><img src="photo.jpg" alt="A photo"><p>After image</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
