---
id: fixture_go_result_warning_kind_image_extraction_failed
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
	}
	result, err := htmd.Convert(`<p>Text<img src="data:BADMIME" alt="broken">end</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
