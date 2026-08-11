---
id: fixture_kotlin_android_options_preprocessing_enabled_false_skips_cleanup
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<nav>NavSection</nav><p>Paragraph</p>", options)
}

```
