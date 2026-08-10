```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<article><h1>Article Title</h1><p>Introduction paragraph.</p><figure><img src="diagram.png" alt="System architecture diagram"><figcaption>Figure 1: System Architecture</figcaption></figure><p>Explanation of the figure.</p></article>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
