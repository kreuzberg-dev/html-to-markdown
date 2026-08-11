---
id: fixture_elixir_twitter_card_tags
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<html><head><meta name=\"twitter:card\" content=\"summary_large_image\"><meta name=\"twitter:site\" content=\"@examplesite\"><meta name=\"twitter:title\" content=\"Twitter Card Title\"><meta name=\"twitter:description\" content=\"Twitter card description.\"><meta name=\"twitter:image\" content=\"https://example.com/twitter-image.jpg\"></head><body><p>Content</p></body></html>")

```
