```kotlin
import io.xberg.android.HtmlToMarkdown
import io.xberg.android.HtmlToMarkdownRsBridgeException

fun main() {
    try {
        val result = HtmlToMarkdown.convert("<h1>Hello</h1>")
        println(result.content)
    } catch (error: HtmlToMarkdownRsBridgeException) {
        System.err.println("Conversion failed: ${error.message}")
    }
}
```
