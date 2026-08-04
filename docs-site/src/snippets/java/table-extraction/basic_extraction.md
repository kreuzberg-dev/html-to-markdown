```java
import io.xberg.htmltomarkdown.HtmlToMarkdown;
import io.xberg.htmltomarkdown.ConversionOptions;
import io.xberg.htmltomarkdown.ConversionResult;
import io.xberg.htmltomarkdown.HtmlToMarkdownRsException;

public class TableExample {
    public static void main(String[] args) throws HtmlToMarkdownRsException {
        String html = """
            <table>
                <tr><th>Name</th><th>Age</th></tr>
                <tr><td>Alice</td><td>30</td></tr>
                <tr><td>Bob</td><td>25</td></tr>
            </table>
            """;

        ConversionOptions options = ConversionOptions.builder()
            .withIncludeDocumentStructure(true)
            .build();
        ConversionResult result = HtmlToMarkdown.convert(html, options);

        for (var table : result.tables()) {
            for (var cell : table.grid().cells()) {
                String prefix = Boolean.TRUE.equals(cell.isHeader()) ? "Header" : "Cell";
                System.out.printf("  %s (r%d,c%d): %s%n", prefix, cell.row(), cell.col(), cell.content());
            }
        }
    }
}
```
