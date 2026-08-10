```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{preprocessing: %{"remove_forms" => false}}
result = HtmlToMarkdown.convert("<form><label for=\"name\">Name:</label><input type=\"text\" id=\"name\" placeholder=\"Enter name\"></form>", options_value)

```
