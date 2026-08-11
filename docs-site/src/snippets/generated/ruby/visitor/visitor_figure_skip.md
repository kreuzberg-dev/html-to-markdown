---
id: fixture_ruby_visitor_figure_skip
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_figure_start(*args)
    'Skip'
  end
end.new
result = HtmlToMarkdown.convert('<p>See the chart below:</p><figure><img src="chart.svg"><figcaption>Revenue Trends 2020-2024</figcaption></figure><p>As shown in the chart above.</p>', visitor)

```
