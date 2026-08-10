```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<p>The <abbr title="World Wide Web">WWW</abbr> is global.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
