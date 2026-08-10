```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<body><div role="complementary">Sidebar</div><p>Primary text</p></body>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
