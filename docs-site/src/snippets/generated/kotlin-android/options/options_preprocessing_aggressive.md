```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<nav>Menu</nav><article><h1>Title</h1><p>Content</p></article><aside>Sidebar</aside><footer>Footer</footer>", options)
}

```
