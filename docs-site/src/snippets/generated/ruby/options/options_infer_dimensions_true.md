```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>With dims: <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==" alt="pixel"></p>', HtmlToMarkdownRs::ConversionOptions.new(extract_images: true, infer_dimensions: true))

```
