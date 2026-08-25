{# Heading guarded on the same set the if-chain below covers: swift, dart, zig, kotlin_android,
   wasm and c have no branch, so the heading used to render with an empty body. ~keep #}
{% if language in ['python', 'typescript', 'ruby', 'php', 'go', 'java', 'csharp', 'elixir', 'r'] %}
### Core Function
{% endif %}

{% if language == 'python' %}
**`convert(html: str, options?: ConversionOptions, visitor?: object) -> ConversionResult`**

Converts HTML to Markdown. Returns a `ConversionResult` object with all results in a single call.

```python
from html_to_markdown import convert, ConversionOptions

result = convert(html)
markdown = result.content           # Converted Markdown string
metadata = result.metadata          # Metadata (when extract_metadata=True)
tables   = result.tables            # Structured table data
document = result.document          # Document-level info
images   = result.metadata.images   # Extracted images
warnings = result.warnings          # Any conversion warnings
```

{% elif language == 'typescript' %}
**`convert(html: string, options?: ConversionOptions): ConversionResult`**

Converts HTML to Markdown. Returns a `ConversionResult` object with all results in a single call.

```typescript
import { convert, ConversionOptions } from "@xberg-io/html-to-markdown";

const result = convert(html);
const markdown = result.content; // Converted Markdown string
const metadata = result.metadata; // Metadata (when extractMetadata: true)
const tables = result.tables; // Structured table data (needs includeDocumentStructure: true)
const document = result.document; // Document-level info (needs includeDocumentStructure: true)
const warnings = result.warnings; // Any conversion warnings
```

{% elif language == 'ruby' %}
**`convert(html, options_or_visitor = nil) -> ConversionResult`**

Converts HTML to Markdown. Returns a `ConversionResult` object with accessor methods for all results in a single call. The second positional argument accepts either an options `Hash`/`ConversionOptions` or a visitor object — not both.

```ruby
require 'html_to_markdown'

result = HtmlToMarkdown.convert(html)
markdown = result.content       # Converted Markdown string
metadata = result.metadata      # Metadata (when extract_metadata: true)
tables   = result.tables        # Structured table data (when include_document_structure: true)
document = result.document      # Document-level info
warnings = result.warnings      # Any conversion warnings
```

{% elif language == 'php' %}
**`HtmlToMarkdownApi::convert(string $html, ?ConversionOptions $options = null): ConversionResult`**

Converts HTML to Markdown. Returns a `ConversionResult` object with all results in a single call.

```php
<?php
use HtmlToMarkdown\HtmlToMarkdownApi;

$result   = HtmlToMarkdownApi::convert($html);
$markdown = $result->content;           // Converted Markdown string
$metadata = $result->getMetadata();     // Metadata
$tables   = $result->getTables();       // Structured table data (needs includeDocumentStructure: true)
$document = $result->getDocument();     // Document-level info (needs includeDocumentStructure: true)
$warnings = $result->getWarnings();     // Any conversion warnings
```

{% elif language == 'go' %}
**`Convert(html string, options *ConversionOptions) (*ConversionResult, error)`**

Converts HTML to Markdown. Returns a `ConversionResult` struct with all results in a single call.

```go
result, err := htmltomarkdown.Convert(html, nil)
markdown := result.Content  // *string - converted Markdown
metadata := result.Metadata // HTMLMetadata
tables   := result.Tables   // []TableData
```

{% elif language == 'java' %}
**`HtmlToMarkdown.convert(String html) : ConversionResult`**
**`HtmlToMarkdown.convert(String html, ConversionOptions options) : ConversionResult`**

Converts HTML to Markdown. Returns a `ConversionResult` record with all results in a single call.

```java
ConversionResult result = HtmlToMarkdown.convert(html);
String   markdown = result.content();   // Converted Markdown string
HtmlMetadata metadata = result.metadata();
List<TableData> tables = result.tables();
```

{% elif language == 'csharp' %}
**`{{ csharp_wrapper_class }}.Convert(string html, ConversionOptions? options) : ConversionResult`**

Converts HTML to Markdown. Returns a `ConversionResult` record with all results in a single call.

```csharp
var result   = {{ csharp_wrapper_class }}.Convert(html, null);
var markdown = result.Content;    // Converted Markdown string
var metadata = result.Metadata;
var tables   = result.Tables;      // Populated when ConversionOptions.IncludeDocumentStructure is true
```

{% elif language == 'elixir' %}
**`HtmlToMarkdown.convert(html, options \\ nil) :: {:ok, ConversionResult.t()} | {:error, atom(), String.t()}`**

Converts HTML to Markdown. Returns `{:ok, result}` where result is a struct with all results in a single call.

```elixir
{:ok, result} = HtmlToMarkdown.convert(html)
result.content          # Converted Markdown string
result.metadata         # HtmlMetadata struct (always present; extract_metadata defaults to true)
result.metadata.images  # Extracted images
result.tables           # Table data list (empty unless include_document_structure: true)
result.document         # Document-level structure (nil unless include_document_structure: true)
result.warnings         # Any conversion warnings
```

{% elif language == 'r' %}
**`convert(html, options = NULL)`**

Converts HTML to Markdown. Returns a named list `ConversionResult` with all results in a single call.

```r
result   <- convert(html)
markdown <- result$content    # Converted Markdown string
metadata <- result$metadata   # Metadata (when extract_metadata = TRUE)
tables   <- result$tables     # Table data
```

{% else %}
{% endif %}

### Options

**`ConversionOptions`** – Key configuration fields:

{% if language == 'typescript' %}

- `headingStyle`: Heading format (`"Underlined"` | `"Atx"` | `"AtxClosed"`) — default: `"Atx"`
- `listIndentWidth`: Spaces per indent level — default: `2`
- `bullets`: Bullet characters cycle — default: `"-*+"`
- `wrap`: Enable text wrapping — default: `false`
- `wrapWidth`: Wrap at column — default: `80`
- `codeLanguage`: Default fenced code block language — default: none
- `extractMetadata`: Enable metadata extraction into `result.metadata` — default: `true`
- `outputFormat`: Output markup format (`"Markdown"` | `"Djot"` | `"Plain"`) — default: `"Markdown"`
{% else %}
- `heading_style`: Heading format (`"underlined"` | `"atx"` | `"atx_closed"`) — default: `"atx"`
- `list_indent_width`: Spaces per indent level — default: `2`
- `bullets`: Bullet characters cycle — default: `"-*+"`
- `wrap`: Enable text wrapping — default: `false`
- `wrap_width`: Wrap at column — default: `80`
- `code_language`: Default fenced code block language — default: none
- `extract_metadata`: Enable metadata extraction into `result.metadata` — default: `true`
- `output_format`: Output markup format (`"markdown"` | `"djot"` | `"plain"`) — default: `"markdown"`
{% endif %}
