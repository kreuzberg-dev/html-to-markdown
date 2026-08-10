```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{extract_metadata: true}
result = HtmlToMarkdown.convert("<html><head><title>Links Page</title></head><body><p>Visit <a href=\"https://example.com\">Example</a> or <a href=\"https://docs.example.com\">Docs</a>.</p><p>Also see <a href=\"/relative/path\">relative link</a> and <a href=\"mailto:hello@example.com\">email us</a>.</p></body></html>", options_value)

```
