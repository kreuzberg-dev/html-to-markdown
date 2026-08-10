```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<form><label>Username: <input type=\"text\" name=\"username\" value=\"\"></label><label>Password: <input type=\"password\" name=\"password\"></label></form>", ConversionOptions())
}

```
