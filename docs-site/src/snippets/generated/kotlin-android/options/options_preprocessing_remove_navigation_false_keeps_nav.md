```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<nav>SiteMenu</nav><main><p>MainContent</p></main><aside>SidebarText</aside>", options)
}

```
