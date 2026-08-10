```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<div><form action=\"/submit\" method=\"POST\"><label>Name: <input type=\"text\" name=\"name\"></label><button type=\"submit\">Submit</button></form></div>", ConversionOptions.builder().build());
    }
}

```
