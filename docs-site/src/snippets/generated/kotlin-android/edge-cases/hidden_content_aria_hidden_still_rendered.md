---
id: fixture_kotlin_android_hidden_content_aria_hidden_still_rendered
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>visible</p><div aria-hidden=\"true\">still shown</div><p>also visible</p>", ConversionOptions())
}

```
