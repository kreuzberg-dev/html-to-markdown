---
id: fixture_go_visitor_video_custom
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
	result, err := htmd.Convert(`<p>Watch our tutorial:</p><video src="tutorial.mp4" width="320" height="240" controls></video><p>Great content!</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
