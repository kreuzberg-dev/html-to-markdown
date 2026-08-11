---
id: fixture_elixir_metadata_microdata_schema_article
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{extract_metadata: true}
result = HtmlToMarkdown.convert("<html><head><title>Article</title></head><body><article itemscope itemtype=\"https://schema.org/Article\"><h1 itemprop=\"headline\">Breaking News Today</h1><span itemprop=\"author\">Jane Reporter</span><span itemprop=\"datePublished\">2024-04-22</span><div itemprop=\"articleBody\"><p>The article content goes here with important information about the breaking news story.</p></div></article></body></html>", options_value)

```
