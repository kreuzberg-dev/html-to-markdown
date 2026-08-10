```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<form><label>Username: <input type=\"text\" name=\"username\" value=\"\"></label><label>Password: <input type=\"password\" name=\"password\"></label></form>", ConversionOptions.builder().build());
    }
}

```
