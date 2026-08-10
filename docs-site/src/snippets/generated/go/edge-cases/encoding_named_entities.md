```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<p>Em dash&mdash;used for parenthetical remarks&mdash;is common. Ellipsis&hellip; indicates omission. Non-breaking&nbsp;space.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
