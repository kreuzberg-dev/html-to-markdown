```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<p>Embedded map:</p><iframe src="https://maps.example.com/embed" width="400" height="300"></iframe><p>End of map</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
