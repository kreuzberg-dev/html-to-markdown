```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{preprocessing: %{"remove_forms" => false}}
result = HtmlToMarkdown.convert("<form><label>Color:</label><select><option value=\"red\">Red</option><option value=\"blue\" selected>Blue</option><option value=\"green\">Green</option></select></form>", options_value)

```
