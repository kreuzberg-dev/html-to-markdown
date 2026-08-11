---
id: fixture_kotlin_android_options_preprocessing_remove_navigation_false_keeps_nav
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<nav>SiteMenu</nav><main><p>MainContent</p></main><aside>SidebarText</aside>", options)
}

```
