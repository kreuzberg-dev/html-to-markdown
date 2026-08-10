```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<html lang="es"><head><title>Spanish Page</title></head><body><h1>Hola Mundo</h1><p>Este es un documento en español.</p></body></html>', HtmlToMarkdownRs::ConversionOptions.new(extract_metadata: true))

```
