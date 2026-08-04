```go
import (
    "fmt"
    "log"

    htmltomarkdown "github.com/xberg-io/html-to-markdown/packages/go/v3"
)

html := `
<table>
    <tr><th>Name</th><th>Age</th></tr>
    <tr><td>Alice</td><td>30</td></tr>
    <tr><td>Bob</td><td>25</td></tr>
</table>
`

result, err := htmltomarkdown.Convert(html, &htmltomarkdown.ConversionOptions{
    IncludeDocumentStructure: true,
})
if err != nil {
    log.Fatal(err)
}

for _, table := range result.Tables {
    for _, cell := range table.Grid.Cells {
        kind := "Cell"
        if cell.IsHeader {
            kind = "Header"
        }
        fmt.Printf("  %s (r%d,c%d): %s\n", kind, cell.Row, cell.Col, cell.Content)
    }
}
```
