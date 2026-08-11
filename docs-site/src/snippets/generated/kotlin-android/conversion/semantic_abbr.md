---
id: fixture_kotlin_android_semantic_abbr
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>The <abbr title=\"World Wide Web\">WWW</abbr> is global.</p>", ConversionOptions())
}

```
