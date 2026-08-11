```ruby
require 'html_to_markdown'

# Binary data (detected via magic bytes) is rejected before parsing.
html = "%PDF-1.4 not actually HTML"

begin
  result = HtmlToMarkdown.convert(html)
  markdown = result.content
rescue RuntimeError => e
  # Native conversion failures surface as RuntimeError with the Rust error message.
  warn "conversion failed: #{e.message}"
end
```
