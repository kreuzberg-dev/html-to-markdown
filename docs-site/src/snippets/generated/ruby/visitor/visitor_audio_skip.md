```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_audio(ctx, src)
    'Skip'
  end
end.new
result = HtmlToMarkdown.convert('<p>Background music:</p><audio src="music.ogg" autoplay></audio><p>Enjoy!</p>', visitor)

```
