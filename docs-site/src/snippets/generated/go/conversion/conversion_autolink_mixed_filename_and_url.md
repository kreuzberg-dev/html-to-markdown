---
id: fixture_go_conversion_autolink_mixed_filename_and_url
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
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<a href="foobar.png">foobar.png</a> <a href="https://www.heise.de">https://www.heise.de</a>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
