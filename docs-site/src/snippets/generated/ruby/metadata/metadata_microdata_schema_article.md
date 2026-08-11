---
id: fixture_ruby_metadata_microdata_schema_article
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<html><head><title>Article</title></head><body><article itemscope itemtype="https://schema.org/Article"><h1 itemprop="headline">Breaking News Today</h1><span itemprop="author">Jane Reporter</span><span itemprop="datePublished">2024-04-22</span><div itemprop="articleBody"><p>The article content goes here with important information about the breaking news story.</p></div></article></body></html>', HtmlToMarkdownRs::ConversionOptions.new(extract_metadata: true))

```
