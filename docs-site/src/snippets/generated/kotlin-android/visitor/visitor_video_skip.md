---
id: fixture_kotlin_android_visitor_video_skip
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<h2>Demo</h2><video src=\"demo.webm\"></video><p>See the demo above.</p>", ConversionOptions())
}

```
