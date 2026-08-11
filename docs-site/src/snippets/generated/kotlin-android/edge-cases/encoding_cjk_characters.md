---
id: fixture_kotlin_android_encoding_cjk_characters
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>中文内容</p><p>日本語テキスト</p><p>한국어 텍스트</p>", ConversionOptions())
}

```
