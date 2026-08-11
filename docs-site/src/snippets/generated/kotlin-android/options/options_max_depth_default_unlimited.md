---
id: fixture_kotlin_android_options_max_depth_default_unlimited
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<div><div><div><div><p>Deep content</p></div></div></div></div>", ConversionOptions())
}

```
