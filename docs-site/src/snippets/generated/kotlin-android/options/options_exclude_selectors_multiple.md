---
id: fixture_kotlin_android_options_exclude_selectors_multiple
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<body><nav class=\"nav\">Menu</nav><p>Content</p><footer>Footer</footer></body>", options)
}

```
