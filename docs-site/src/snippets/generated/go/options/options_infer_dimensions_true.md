---
id: fixture_go_options_infer_dimensions_true
language: go
target: go
level: typecheck
requires: []
side_effect: safe
---

```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{
		ExtractImages:   true,
		InferDimensions: true,
	}
	result, err := htmd.Convert(`<p>With dims: <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==" alt="pixel"></p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
