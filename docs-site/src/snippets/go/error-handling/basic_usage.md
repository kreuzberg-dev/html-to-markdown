```go
package main

import (
    "fmt"

    htmltomarkdown "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

func main() {
    // Binary data (detected via magic bytes) is rejected before parsing.
    html := "%PDF-1.4 not actually HTML"

    result, err := htmltomarkdown.Convert(html, nil)
    if err != nil {
        // Convert wraps the FFI error as "[<code>] <message>".
        fmt.Println("conversion failed:", err)
        return
    }

    if result.Content != nil {
        fmt.Println(*result.Content)
    }
}
```
