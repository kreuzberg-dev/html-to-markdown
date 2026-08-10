```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<h2>Demo</h2><video src="demo.webm"></video><p>See the demo above.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
