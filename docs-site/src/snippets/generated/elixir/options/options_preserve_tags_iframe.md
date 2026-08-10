```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{preserve_tags: ["iframe"]}
result = HtmlToMarkdown.convert("<p>Before</p><iframe src='video.html' width='560'></iframe><p>After</p>", options_value)

```
