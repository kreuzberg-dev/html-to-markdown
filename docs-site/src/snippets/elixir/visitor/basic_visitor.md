# Visitor Pattern - Elixir

Customize HTML to Markdown conversion by passing a visitor map under
the `:visitor` key of `HtmlToMarkdown.convert/2`'s options. Each entry
maps a callback **atom** (e.g. `:handle_link`) to a one-arity function
that receives a native, string-keyed arguments map built on the Rust side.

The bridge spawns a system thread for the conversion, then sends
`{:visitor_callback, ref_id, callback_name, args_json}` messages back
to the calling process. `HtmlToMarkdown.convert/2` runs a receive loop
that dispatches each message against your visitor map and calls
`HtmlToMarkdown.Native.visitor_reply/2` to unblock the worker.

## Basic Visitor Example

```elixir
visitor = %{
  :handle_link => fn args ->
    text = Map.get(args, "text", "")
    {:custom, text}
  end,
  :handle_image => fn _args -> :skip end,
  :handle_text => fn _args -> :continue end
}

html = "<p>Visit <a href='https://example.com'>our site</a> for more!</p>"
{:ok, result} = HtmlToMarkdown.convert(html, %{visitor: visitor})
IO.puts(result.content)
# => Visit our site for more!
```

## Visitor Return Values

Each function must return one of:

- `:continue` — proceed with default conversion
- `:skip` — omit this element entirely
- `:preserve_html` — include the raw HTML verbatim
- `{:custom, markdown_string}` — replace this element's output with the given string
- A bare string — treated as a custom replacement

Anything else falls back to `:continue`.

## Callback Names

Callbacks are keyed by atom and use the `handle_` prefix (the bridge
translates the Rust `visit_X` trait methods to `:handle_X` over the
wire). Frequently overridden:

- `:handle_text` — text nodes: `%{"ctx" => …, "text" => "…"}` (called ~100+ times per document; keep it cheap)
- `:handle_link` — `<a>` elements: `%{"ctx" => …, "href" => "…", "text" => "…", "title" => …}`
- `:handle_image` — `<img>` elements: `%{"ctx" => …, "src" => "…", "alt" => "…", "title" => …}`
- `:handle_heading` — headings: `%{"ctx" => …, "level" => 1, "text" => "…", "id" => …}`
- `:handle_code_block` — `<pre><code>`: `%{"ctx" => …, "lang" => …, "code" => "…"}`
- `:handle_element_start` / `:handle_element_end` — generic enter/leave hooks

Omit a callback to fall through to the default Rust implementation.

## Node Context

The `"ctx"` value in every callback arg map is a `%HtmlToMarkdown.NodeContext{}` struct:

```elixir
%HtmlToMarkdown.NodeContext{
  node_type: :link,
  tag_name: "a",
  depth: 2,
  index_in_parent: 0,
  parent_tag: "p",
  is_inline: true
}
```

## Combining Options and Visitor

Pass the visitor under the `:visitor` key alongside any other
`ConversionOptions` fields:

```elixir
{:ok, result} = HtmlToMarkdown.convert(html, %{
  visitor: %{:handle_link => fn _ -> :skip end},
  output_format: "github",
  extract_metadata: true
})
```

`HtmlToMarkdown.convert/2` pops the `:visitor` key, JSON-encodes the
remaining options, and dispatches to `convert_with_visitor`.
