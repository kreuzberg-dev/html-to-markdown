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
	result, err := htmd.Convert(`<form><label for="name">Name:</label><input type="text" id="name" placeholder="Enter name"></form>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
