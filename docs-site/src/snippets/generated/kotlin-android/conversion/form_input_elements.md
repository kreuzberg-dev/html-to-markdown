```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<form><label for=\"name\">Name:</label><input type=\"text\" id=\"name\" placeholder=\"Enter name\"></form>", options)
}

```
