```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{preprocessing: %{"remove_forms" => true}}
result = HtmlToMarkdown.convert("<p>Before</p><form><input type='text'/><button>Submit</button></form><p>After</p>", options_value)

```
