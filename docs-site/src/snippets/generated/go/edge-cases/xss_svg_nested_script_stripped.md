```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<p>Before SVG.</p><svg xmlns="http://www.w3.org/2000/svg"><script>alert('svg-xss')</script><text>SVG text</text></svg><p>After SVG.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
