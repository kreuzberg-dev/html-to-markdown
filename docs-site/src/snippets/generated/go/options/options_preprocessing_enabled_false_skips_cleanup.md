```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{
		Preprocessing: &htmd.PreprocessingOptions{
		Enabled: false,
	},
	}
	result, err := htmd.Convert(`<nav>NavSection</nav><p>Paragraph</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
