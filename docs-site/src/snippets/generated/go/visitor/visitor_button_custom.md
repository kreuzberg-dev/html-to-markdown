```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<p>Confirm action: <button type="submit">Click me</button> or <button type="reset">Cancel</button></p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
