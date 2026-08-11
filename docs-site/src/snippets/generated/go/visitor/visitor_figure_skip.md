---
id: fixture_go_visitor_figure_skip
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
	result, err := htmd.Convert(`<p>See the chart below:</p><figure><img src="chart.svg"><figcaption>Revenue Trends 2020-2024</figcaption></figure><p>As shown in the chart above.</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
