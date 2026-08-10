```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<h3>Reviews</h3><iframe src="https://widget.example.com/reviews"></iframe><p>See reviews from our partners.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
