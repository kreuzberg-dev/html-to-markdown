---
id: fixture_go_options_preprocessing_remove_navigation_false_keeps_nav
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
		Preprocessing: &htmd.PreprocessingOptions{
		RemoveNavigation: false,
	},
	}
	result, err := htmd.Convert(`<nav>SiteMenu</nav><main><p>MainContent</p></main><aside>SidebarText</aside>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
