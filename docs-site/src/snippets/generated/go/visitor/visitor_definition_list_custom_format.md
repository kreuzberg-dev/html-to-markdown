```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<dl><dt>Python</dt><dd>A high-level programming language</dd><dt>JavaScript</dt><dd>A scripting language for web browsers</dd></dl>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
