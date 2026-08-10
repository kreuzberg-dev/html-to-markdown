```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Sign up:</p><input type=\"text\" name=\"email\" placeholder=\"your@email.com\"><input type=\"checkbox\" name=\"agree\"><p>Continue</p>", ConversionOptions())
}

```
