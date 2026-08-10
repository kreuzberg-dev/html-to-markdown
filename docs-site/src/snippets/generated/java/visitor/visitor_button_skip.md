```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Actions available: <button>Save</button> <button>Delete</button> <button>Export</button></p>", ConversionOptions.builder().build());
    }
}

```
