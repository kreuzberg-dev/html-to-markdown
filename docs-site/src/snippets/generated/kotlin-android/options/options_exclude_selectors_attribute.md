```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<body><div role=\"complementary\">Sidebar</div><p>Primary text</p></body>", options)
}

```
