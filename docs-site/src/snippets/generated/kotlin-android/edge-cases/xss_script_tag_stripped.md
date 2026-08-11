---
id: fixture_kotlin_android_xss_script_tag_stripped
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Safe content.</p><script>alert('xss')</script><p>More safe content.</p>", ConversionOptions())
}

```
