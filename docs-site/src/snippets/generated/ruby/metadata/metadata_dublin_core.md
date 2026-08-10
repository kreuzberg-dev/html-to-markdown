```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<html><head><title>Scholarly Work</title><meta name="DC.title" content="Principles of Knowledge Management"><meta name="DC.creator" content="Dr. Alice Johnson"><meta name="DC.date" content="2023-06-15"><meta name="DC.subject" content="Knowledge Management"><meta name="DC.publisher" content="Academic Press"></head><body><p>This is a scholarly article.</p></body></html>', HtmlToMarkdownRs::ConversionOptions.new(extract_metadata: true))

```
