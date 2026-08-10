```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func ptr[T any](value T) *T { return &value }
func main() {
	options := htmd.ConversionOptions{
		Preprocessing: &htmd.PreprocessingOptions{
		Preset: ptr(htmd.PreprocessingPreset(`Minimal`)),
	},
	}
	result, err := htmd.Convert(`<nav>Navigation</nav><p>Content</p><footer>Footer</footer>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
