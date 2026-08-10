```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Before form</p><form><input type=\"email\" name=\"email\"></form><p>After form</p>", ConversionOptions())
}

```
