---
id: fixture_kotlin_android_options_exclude_selectors_attribute
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<body><div role=\"complementary\">Sidebar</div><p>Primary text</p></body>", options)
}

```
