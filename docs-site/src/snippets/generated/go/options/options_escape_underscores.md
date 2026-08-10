```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{
		EscapeUnderscores: true,
	}
	result, err := htmd.Convert(`<p>The variable_name is defined.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
