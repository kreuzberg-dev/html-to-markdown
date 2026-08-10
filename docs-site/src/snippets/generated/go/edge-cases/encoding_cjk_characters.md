```go title="Go"
package main

import (
	"fmt"
	htmd "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
	options := htmd.ConversionOptions{}
	result, err := htmd.Convert(`<p>中文内容</p><p>日本語テキスト</p><p>한국어 텍스트</p>`, options)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
