```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{
		Preprocessing: &htmd.PreprocessingOptions{
		RemoveForms: true,
	},
	}
	result, err := htmd.Convert(`<p>Before</p><form><input type='text'/><button>Submit</button></form><p>After</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
