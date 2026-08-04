# Metadata Extraction - Elixir

Extract structured metadata from HTML documents during conversion.

## Basic Metadata Extraction

Use `convert/2` with `extract_metadata: true` in options to extract document metadata alongside Markdown:

```elixir
html = """
<html>
  <head>
    <title>Example</title>
    <meta name="description" content="Demo page">
  </head>
  <body>
    <h1 id="welcome">Welcome</h1>
    <a href="https://example.com" rel="nofollow external">Example link</a>
  </body>
</html>
"""

opts = %HtmlToMarkdown.ConversionOptions{extract_metadata: true}
{:ok, result} = HtmlToMarkdown.convert(html, opts)

IO.inspect(result.metadata.document.title)                  # "Example"
IO.inspect(result.metadata.headers |> hd() |> Map.get(:text))     # "Welcome"
IO.inspect(result.metadata.links   |> hd() |> Map.get(:link_type)) # :external
```

## Extracted Metadata Structure

The metadata map includes:

- **Document**: Title and meta tags from `<head>`
- **Headers**: All headings extracted with level, text, and optional ID
- **Links**: All links with href, text, rel attributes, and link_type classification
- **Images**: Image sources and alt text
- **Forms**: Form action and method data
- **Other**: Tables, code blocks, and additional structural information
