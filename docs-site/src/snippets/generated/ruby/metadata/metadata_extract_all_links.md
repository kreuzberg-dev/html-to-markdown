```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<html><head><title>Links Page</title></head><body><p>Visit <a href="https://example.com">Example</a> or <a href="https://docs.example.com">Docs</a>.</p><p>Also see <a href="/relative/path">relative link</a> and <a href="mailto:hello@example.com">email us</a>.</p></body></html>', HtmlToMarkdownRs::ConversionOptions.new(extract_metadata: true))

```
