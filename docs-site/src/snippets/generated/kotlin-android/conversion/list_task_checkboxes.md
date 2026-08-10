```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<ul><li><input type=\"checkbox\" checked> Done task</li><li><input type=\"checkbox\"> Pending task</li></ul>", ConversionOptions())
}

```
