```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<table><tr><td>Product</td><td>Price</td></tr><tr><td>Apple</td><td>1.00</td></tr></table>", ConversionOptions())
}

```
