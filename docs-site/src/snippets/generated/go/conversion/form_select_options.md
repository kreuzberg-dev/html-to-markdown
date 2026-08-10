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
	result, err := htmd.Convert(`<form><label>Color:</label><select><option value="red">Red</option><option value="blue" selected>Blue</option><option value="green">Green</option></select></form>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
