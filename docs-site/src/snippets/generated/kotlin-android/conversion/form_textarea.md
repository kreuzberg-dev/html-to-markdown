---
id: fixture_kotlin_android_form_textarea
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<form><label>Message:</label><textarea>Default text content</textarea></form>", options)
}

```
