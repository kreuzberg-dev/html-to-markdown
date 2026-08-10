```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<body><nav class=\"nav\">Menu</nav><p>Content</p><footer>Footer</footer></body>", options)
}

```
