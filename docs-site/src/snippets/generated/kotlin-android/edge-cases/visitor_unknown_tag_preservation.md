```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<article><p>Article text</p><x-custom>Custom element with content</x-custom><p>More article text</p></article>", ConversionOptions())
}

```
