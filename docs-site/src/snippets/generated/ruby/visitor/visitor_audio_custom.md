```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_audio(ctx, src)
    { Custom: '[AUDIO: podcast.mp3]' }
  end
end.new
result = HtmlToMarkdown.convert('<p>Listen to this: <audio src="podcast.mp3" controls></audio></p>', visitor)

```
