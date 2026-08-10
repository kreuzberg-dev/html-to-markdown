```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<figure><img src=\"sunset.jpg\" alt=\"A sunset\"><figcaption>Beautiful sunset over the ocean</figcaption></figure>", ConversionOptions())
}

```
