```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<html><head><title>Company</title></head><body><div itemscope itemtype="https://schema.org/Organization"><span itemprop="name">Acme Corp</span><span itemprop="foundingDate">2020</span><span itemprop="url">https://acmecorp.example.com</span><span itemprop="logo">https://acmecorp.example.com/logo.png</span></div></body></html>', HtmlToMarkdownRs::ConversionOptions.new(extract_metadata: true))

```
