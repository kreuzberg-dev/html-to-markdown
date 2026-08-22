```kotlin
import io.xberg.android.ConversionOptions
import io.xberg.android.HtmlToMarkdown

fun main() {
    val html = """
        <table>
          <tr><th>Name</th><th>Age</th></tr>
          <tr><td>Alice</td><td>30</td></tr>
          <tr><td>Bob</td><td>25</td></tr>
        </table>
    """.trimIndent()

    // Table extraction requires `includeDocumentStructure = true` — without it,
    // `result.tables` is always empty even for table-heavy HTML.
    val options = ConversionOptions(includeDocumentStructure = true)
    val result = HtmlToMarkdown.convert(html, options)

    for (table in result.tables) {
        println(table.markdown)
        for (cell in table.grid.cells) {
            val kind = if (cell.isHeader) "Header" else "Cell"
            println("  $kind (r${cell.row},c${cell.col}): ${cell.content}")
        }
    }
}
```
