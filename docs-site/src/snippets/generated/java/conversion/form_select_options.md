```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var optionsJson = "{\"preprocessing\":{\"remove_forms\":false}}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<form><label>Color:</label><select><option value=\"red\">Red</option><option value=\"blue\" selected>Blue</option><option value=\"green\">Green</option></select></form>", options);
    }
}

```
