```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{
		Preprocessing: &htmd.PreprocessingOptions{
		RemoveForms: false,
	},
	}
	result, err := htmd.Convert(`<form><label>Message:</label><textarea>Default text content</textarea></form>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
