```java
import io.xberg.htmltomarkdown.HtmlToMarkdown;
import io.xberg.htmltomarkdown.ConversionResult;
import io.xberg.htmltomarkdown.InvalidInputException;
import io.xberg.htmltomarkdown.HtmlToMarkdownRsException;

public class Example {
    public static void main(String[] args) {
        // Binary data (detected via magic bytes) is rejected before parsing.
        String html = "%PDF-1.4 not actually HTML";

        try {
            ConversionResult result = HtmlToMarkdown.convert(html);
            System.out.println(result.content());
        } catch (InvalidInputException e) {
            System.err.println("invalid input: " + e.getMessage());
        } catch (HtmlToMarkdownRsException e) {
            System.err.println("conversion failed: " + e.getMessage());
        }
    }
}
```
