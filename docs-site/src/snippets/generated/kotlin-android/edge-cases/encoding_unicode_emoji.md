---
id: fixture_kotlin_android_encoding_unicode_emoji
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Hello 🌍 World 🚀</p><p>Stars: ⭐ ✨</p>", ConversionOptions())
}

```
