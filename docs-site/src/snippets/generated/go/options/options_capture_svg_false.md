---
id: fixture_go_options_capture_svg_false
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
		ExtractImages: true,
		CaptureSvg:    false,
	}
	result, err := htmd.Convert(`<p>Below SVG:</p><svg xmlns="http://www.w3.org/2000/svg" width="10" height="10"><rect width="10" height="10" fill="red"/></svg>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
