```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<div><form action=\"/submit\" method=\"POST\"><label>Name: <input type=\"text\" name=\"name\"></label><button type=\"submit\">Submit</button></form></div>", ConversionOptions())
}

```
