```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Image: <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==" alt="pixel"></p>', HtmlToMarkdownRs::ConversionOptions.new(extract_images: true, max_image_size: 10485760))

```
