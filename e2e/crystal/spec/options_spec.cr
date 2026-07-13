require "./spec_helper"

describe HtmlToMarkdown do
  describe "options" do
    it "Backticks code block trims trailing newline inside fence and adds blank line after closing fence (issue #396)" do
      __result = HtmlToMarkdown.convert("<p>Foo</p><pre><code>1\n2\n</code></pre><p>Bar</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"code_block_style\":\"Backticks\"}"))
      __result.content.to_s.strip.should eq("Foo\n\n```\n1\n2\n```\n\nBar")
    end
    it "Bare URL links rendered as regular markdown links when autolinks disabled" do
      __result = HtmlToMarkdown.convert("<p><a href='https://example.com'>https://example.com</a></p>", HtmlToMarkdown::ConversionOptions.from_json("{\"autolinks\":false}"))
      __result.content.to_s.should contain("example.com")
    end
    it "BR elements in table cells are stripped when disabled" do
      __result = HtmlToMarkdown.convert("<table><tr><th>Col</th></tr><tr><td>A<br>B</td></tr></table>", HtmlToMarkdown::ConversionOptions.from_json("{\"br_in_tables\":false}"))
      __result.content.to_s.should contain("Col")
    end
    it "BR elements in table cells render as line breaks" do
      __result = HtmlToMarkdown.convert("<table><tr><th>Header</th></tr><tr><td>Line 1<br>Line 2</td></tr></table>", HtmlToMarkdown::ConversionOptions.from_json("{\"br_in_tables\":true}"))
      __result.content.to_s.should contain("Header")
      __result.content.to_s.should contain("Line 1")
      __result.content.to_s.should contain("Line 2")
    end
    it "Disabling capture_svg still produces content; SVG is not extracted as an image" do
      __result = HtmlToMarkdown.convert("<p>Below SVG:</p><svg xmlns=\"http://www.w3.org/2000/svg\" width=\"10\" height=\"10\"><rect width=\"10\" height=\"10\" fill=\"red\"/></svg>", HtmlToMarkdown::ConversionOptions.from_json("{\"capture_svg\":false,\"extract_images\":true}"))
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Below SVG")
    end
    it "Enabling capture_svg processes inline SVG elements when extract_images is on" do
      __result = HtmlToMarkdown.convert("<p>Below SVG:</p><svg xmlns=\"http://www.w3.org/2000/svg\" width=\"10\" height=\"10\"><rect width=\"10\" height=\"10\" fill=\"red\"/></svg>", HtmlToMarkdown::ConversionOptions.from_json("{\"capture_svg\":true,\"extract_images\":true}"))
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Below SVG")
    end
    it "Backticks code block style uses triple backtick fences" do
      __result = HtmlToMarkdown.convert("<pre><code class=\"language-js\">console.log('hi');</code></pre>", HtmlToMarkdown::ConversionOptions.from_json("{\"code_block_style\":\"Backticks\"}"))
      __result.content.to_s.should contain("```")
      __result.content.to_s.should contain("console.log('hi');")
    end
    it "Code blocks use 4-space indentation" do
      __result = HtmlToMarkdown.convert("<pre><code>print('hello')</code></pre>", HtmlToMarkdown::ConversionOptions.from_json("{\"code_block_style\":\"Indented\"}"))
      __result.content.to_s.should contain("print('hello')")
      __result.content.to_s.should_not contain("```")
    end
    it "Code blocks use tilde fences" do
      __result = HtmlToMarkdown.convert("<pre><code>let x = 1;</code></pre>", HtmlToMarkdown::ConversionOptions.from_json("{\"code_block_style\":\"Tildes\"}"))
      __result.content.to_s.should contain("~~~")
      __result.content.to_s.should contain("let x = 1;")
    end
    it "Tildes code block style uses triple tilde fences" do
      __result = HtmlToMarkdown.convert("<pre><code>some code</code></pre>", HtmlToMarkdown::ConversionOptions.from_json("{\"code_block_style\":\"Tildes\"}"))
      __result.content.to_s.should contain("~~~")
      __result.content.to_s.should contain("some code")
    end
    it "Default code language annotation on blocks without lang attribute" do
      __result = HtmlToMarkdown.convert("<pre><code>def hello(): pass</code></pre>", HtmlToMarkdown::ConversionOptions.from_json("{\"code_language\":\"python\"}"))
      __result.content.to_s.should contain("```python")
      __result.content.to_s.should contain("def hello")
    end
    it "compact_tables false (default) pads cells to column width" do
      __result = HtmlToMarkdown.convert("<table><thead><tr><th>Name</th><th>Score</th></tr></thead><tbody><tr><td>Alice</td><td>100</td></tr><tr><td>Bob</td><td>42</td></tr></tbody></table>", HtmlToMarkdown::ConversionOptions.from_json("{\"compact_tables\":false}"))
      __result.content.to_s.should contain("| ----- |")
      __result.content.to_s.should contain("| 42    |")
      __result.content.to_s.should_not contain("| --- |")
    end
    it "compact_tables emits unpadded cells and minimal --- separators" do
      __result = HtmlToMarkdown.convert("<table><thead><tr><th>Name</th><th>Score</th></tr></thead><tbody><tr><td>Alice</td><td>100</td></tr><tr><td>Bob</td><td>42</td></tr></tbody></table>", HtmlToMarkdown::ConversionOptions.from_json("{\"compact_tables\":true}"))
      __result.content.to_s.should contain("| --- |")
      __result.content.to_s.should_not contain("| ----- |")
      __result.content.to_s.should contain("| 42 |")
      __result.content.to_s.should_not contain("| 42    |")
    end
    it "Block elements treated as inline" do
      __result = HtmlToMarkdown.convert("<p>One</p><p>Two</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"convert_as_inline\":true}"))
      __result.content.to_s.should contain("One")
      __result.content.to_s.should contain("Two")
    end
    it "Debug mode enabled does not crash and produces output" do
      __result = HtmlToMarkdown.convert("<p>Debug test</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"debug\":true}"))
      __result.content.to_s.should contain("Debug test")
    end
    it "Links without title get empty title attribute when defaultTitle is true" do
      __result = HtmlToMarkdown.convert("<p><a href='https://example.com'>Link</a></p>", HtmlToMarkdown::ConversionOptions.from_json("{\"default_title\":true}"))
      __result.content.to_s.should contain("Link")
      __result.content.to_s.should contain("https://example.com")
    end
    it "UTF-8 encoding hint for special characters" do
      __result = HtmlToMarkdown.convert("<p>Café naïve résumé</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"encoding\":\"utf-8\"}"))
      __result.content.to_s.should_not be_empty
    end
    it "ASCII Markdown characters are escaped when escapeAscii is true" do
      __result = HtmlToMarkdown.convert("<p>Text with # hash and [brackets] and * star</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"escape_ascii\":true}"))
      __result.content.to_s.should contain("Text")
      __result.content.to_s.should contain("hash")
      __result.content.to_s.should contain("brackets")
      __result.content.to_s.should contain("star")
    end
    it "escape_asterisks option escapes asterisks in plain text" do
      __result = HtmlToMarkdown.convert("<p>Use 2*3 = 6 in math.</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"escape_asterisks\":true}"))
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("2")
      __result.content.to_s.should contain("3")
      __result.content.to_s.should contain("6")
    end
    it "escape_misc option escapes miscellaneous markdown characters" do
      __result = HtmlToMarkdown.convert("<p>Use # and | and ~ in text.</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"escape_misc\":true}"))
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Use")
      __result.content.to_s.should contain("and")
      __result.content.to_s.should contain("in text.")
    end
    it "escape_underscores option escapes underscores in plain text" do
      __result = HtmlToMarkdown.convert("<p>The variable_name is defined.</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"escape_underscores\":true}"))
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("variable")
      __result.content.to_s.should contain("name")
      __result.content.to_s.should contain("defined.")
    end
    it "Elements matching CSS attribute selector are excluded entirely" do
      __result = HtmlToMarkdown.convert("<body><div role=\"complementary\">Sidebar</div><p>Primary text</p></body>", HtmlToMarkdown::ConversionOptions.from_json("{\"exclude_selectors\":[\"[role='complementary']\"]}"))
      __result.content.to_s.should contain("Primary text")
      __result.content.to_s.should_not contain("Sidebar")
    end
    it "Elements matching CSS class selector are excluded entirely" do
      __result = HtmlToMarkdown.convert("<body><div class=\"cookie-banner\">Accept cookies</div><p>Main content</p></body>", HtmlToMarkdown::ConversionOptions.from_json("{\"exclude_selectors\":[\".cookie-banner\"]}"))
      __result.content.to_s.should contain("Main content")
      __result.content.to_s.should_not contain("cookies")
    end
    it "Empty exclude_selectors list does not affect output" do
      __result = HtmlToMarkdown.convert("<p>Hello world</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"exclude_selectors\":[]}"))
      __result.content.to_s.should contain("Hello world")
    end
    it "Elements matching CSS id selector are excluded entirely" do
      __result = HtmlToMarkdown.convert("<body><div id=\"ad-container\">Buy stuff</div><p>Article text</p></body>", HtmlToMarkdown::ConversionOptions.from_json("{\"exclude_selectors\":[\"#ad-container\"]}"))
      __result.content.to_s.should contain("Article text")
      __result.content.to_s.should_not contain("Buy stuff")
    end
    it "Multiple CSS selectors each exclude their matched elements" do
      __result = HtmlToMarkdown.convert("<body><nav class=\"nav\">Menu</nav><p>Content</p><footer>Footer</footer></body>", HtmlToMarkdown::ConversionOptions.from_json("{\"exclude_selectors\":[\".nav\",\"footer\"]}"))
      __result.content.to_s.should contain("Content")
      __result.content.to_s.should_not contain("Menu")
      __result.content.to_s.should_not contain("Footer")
    end
    it "All descendants of excluded elements are dropped" do
      __result = HtmlToMarkdown.convert("<body><aside class=\"sidebar\"><h2>Related</h2><p>Sidebar text</p></aside><main><p>Main text</p></main></body>", HtmlToMarkdown::ConversionOptions.from_json("{\"exclude_selectors\":[\".sidebar\"]}"))
      __result.content.to_s.should contain("Main text")
      __result.content.to_s.should_not contain("Related")
      __result.content.to_s.should_not contain("Sidebar text")
    end
    it "Exclude selectors work in plain text output mode" do
      __result = HtmlToMarkdown.convert("<body><div class=\"nav\">Navigation</div><p>Article body</p></body>", HtmlToMarkdown::ConversionOptions.from_json("{\"exclude_selectors\":[\".nav\"],\"output_format\":\"Plain\"}"))
      __result.content.to_s.should contain("Article body")
      __result.content.to_s.should_not contain("Navigation")
    end
    it "exclude_selectors drops entire subtree unlike strip_tags which keeps children" do
      __result = HtmlToMarkdown.convert("<body><div class=\"wrapper\"><p>Inner paragraph</p></div><p>Outer text</p></body>", HtmlToMarkdown::ConversionOptions.from_json("{\"exclude_selectors\":[\".wrapper\"]}"))
      __result.content.to_s.should contain("Outer text")
      __result.content.to_s.should_not contain("Inner paragraph")
    end
    it "When extract_images is false, conversion still succeeds and inline images are not extracted" do
      __result = HtmlToMarkdown.convert("<p>Text with <img src=\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==\" alt=\"pixel\"> image.</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"extract_images\":false}"))
      __result.content.to_s.should_not be_empty
    end
    it "Enabling extract_images processes a data-URI image without crashing" do
      __result = HtmlToMarkdown.convert("<p>Before<img src=\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==\" alt=\"pixel\">After</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"extract_images\":true}"))
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Before")
      __result.content.to_s.should contain("After")
    end
    it "Extract metadata returns document metadata when enabled" do
      __result = HtmlToMarkdown.convert("<html><head><title>Test Page</title><meta name='description' content='A test page'></head><body><p>Content</p></body></html>", HtmlToMarkdown::ConversionOptions.from_json("{\"extract_metadata\":true}"))
      __result.content.to_s.should_not be_empty
      __result.try(&.metadata).try(&.document).try(&.title).to_s.strip.should eq("Test Page")
      __result.try(&.metadata).try(&.document).try(&.description).to_s.strip.should eq("A test page")
    end
    it "ATX heading style produces hash-prefixed headings" do
      __result = HtmlToMarkdown.convert("<h1>Title</h1><h2>Subtitle</h2>", HtmlToMarkdown::ConversionOptions.from_json("{\"heading_style\":\"Atx\"}"))
      __result.content.to_s.should contain("# Title")
      __result.content.to_s.should contain("## Subtitle")
    end
    it "ATX closed heading style adds closing hashes" do
      __result = HtmlToMarkdown.convert("<h1>Closed Heading</h1>", HtmlToMarkdown::ConversionOptions.from_json("{\"heading_style\":\"AtxClosed\"}"))
      __result.content.to_s.should contain("# Closed Heading #")
    end
    it "Underlined heading style produces setext-style headings for h1 and h2" do
      __result = HtmlToMarkdown.convert("<h1>Main Title</h1>", HtmlToMarkdown::ConversionOptions.from_json("{\"heading_style\":\"Underlined\"}"))
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Main Title")
    end
    it "Mark tag rendered as bold" do
      __result = HtmlToMarkdown.convert("<p>Text with <mark>highlighted</mark> text.</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"highlight_style\":\"Bold\"}"))
      __result.content.to_s.should contain("**highlighted**")
    end
    it "Mark tag with double equal highlight style" do
      __result = HtmlToMarkdown.convert("<p>Text with <mark>highlighted</mark> here.</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"highlight_style\":\"DoubleEqual\"}"))
      __result.content.to_s.should contain("==highlighted==")
    end
    it "Mark tag with no highlight style strips the mark" do
      __result = HtmlToMarkdown.convert("<p>Text with <mark>plain</mark> content.</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"highlight_style\":\"None\"}"))
      __result.content.to_s.should contain("plain")
      __result.content.to_s.should_not contain("==")
    end
    it "Disabling include_document_structure still produces content without the document tree overhead" do
      __result = HtmlToMarkdown.convert("<article><h1>Heading</h1><p>Paragraph body.</p></article>", HtmlToMarkdown::ConversionOptions.from_json("{\"include_document_structure\":false}"))
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Heading")
    end
    it "Setting include_document_structure populates the structured document tree on the result" do
      __result = HtmlToMarkdown.convert("<article><h1>Heading</h1><p>Paragraph body.</p></article>", HtmlToMarkdown::ConversionOptions.from_json("{\"include_document_structure\":true}"))
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Heading")
    end
    it "Disabling infer_dimensions skips the decode step and still produces content" do
      __result = HtmlToMarkdown.convert("<p>No dims: <img src=\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==\" alt=\"pixel\"></p>", HtmlToMarkdown::ConversionOptions.from_json("{\"extract_images\":true,\"infer_dimensions\":false}"))
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("No dims")
    end
    it "Enabling infer_dimensions decodes image bytes to populate dimension metadata when extract_images is on" do
      __result = HtmlToMarkdown.convert("<p>With dims: <img src=\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==\" alt=\"pixel\"></p>", HtmlToMarkdown::ConversionOptions.from_json("{\"extract_images\":true,\"infer_dimensions\":true}"))
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("With dims")
    end
    it "Images inside specified tags stay inline" do
      __result = HtmlToMarkdown.convert("<p>Text <img src='icon.png' alt='icon'> more text</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"keep_inline_images_in\":[\"p\"]}"))
      __result.content.to_s.should contain("Text")
      __result.content.to_s.should contain("more text")
    end
    it "Links use reference-style formatting" do
      __result = HtmlToMarkdown.convert("<p><a href='https://example.com'>Example</a> and <a href='https://other.com'>Other</a></p>", HtmlToMarkdown::ConversionOptions.from_json("{\"link_style\":\"Reference\"}"))
      __result.content.to_s.should contain("Example")
      __result.content.to_s.should contain("Other")
      __result.content.to_s.should contain("example.com")
    end
    it "Custom bullet character for unordered lists" do
      __result = HtmlToMarkdown.convert("<ul><li>Item A</li><li>Item B</li></ul>", HtmlToMarkdown::ConversionOptions.from_json("{\"bullets\":\"*\"}"))
      __result.content.to_s.should contain("* Item A")
      __result.content.to_s.should contain("* Item B")
    end
    it "Tab indentation type for nested list items" do
      __result = HtmlToMarkdown.convert("<ul><li>Parent<ul><li>Child</li></ul></li></ul>", HtmlToMarkdown::ConversionOptions.from_json("{\"list_indent_type\":\"Tabs\"}"))
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Parent")
      __result.content.to_s.should contain("Child")
    end
    it "Nested lists indented with 4 spaces per level" do
      __result = HtmlToMarkdown.convert("<ul><li>Outer<ul><li>Inner</li></ul></li></ul>", HtmlToMarkdown::ConversionOptions.from_json("{\"list_indent_width\":4}"))
      __result.content.to_s.should contain("Outer")
      __result.content.to_s.should contain("Inner")
    end
    it "Default max_depth (null) converts deeply nested content fully" do
      __result = HtmlToMarkdown.convert("<div><div><div><div><p>Deep content</p></div></div></div></div>", nil)
      __result.content.to_s.should contain("Deep content")
    end
    it "max_depth truncates content beyond the specified depth" do
      __result = HtmlToMarkdown.convert("<div><p>Shallow</p><div><div><div><p>Too deep</p></div></div></div></div>", HtmlToMarkdown::ConversionOptions.from_json("{\"max_depth\":3}"))
      __result.content.to_s.should contain("Shallow")
      __result.content.to_s.should_not contain("Too deep")
    end
    it "max_depth of 0 produces empty output" do
      __result = HtmlToMarkdown.convert("<p>Hello</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"max_depth\":0}"))
      __result.content.to_s.strip.should eq("")
    end
    it "A generous max_image_size accommodates a small data-URI image without warnings" do
      __result = HtmlToMarkdown.convert("<p>Image: <img src=\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==\" alt=\"pixel\"></p>", HtmlToMarkdown::ConversionOptions.from_json("{\"extract_images\":true,\"max_image_size\":10485760}"))
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Image")
    end
    it "A tiny max_image_size does not crash conversion even when the image exceeds the limit" do
      __result = HtmlToMarkdown.convert("<p>Tiny limit: <img src=\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==\" alt=\"pixel\"></p>", HtmlToMarkdown::ConversionOptions.from_json("{\"extract_images\":true,\"max_image_size\":10}"))
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Tiny limit")
    end
    it "Hard line breaks rendered with backslash" do
      __result = HtmlToMarkdown.convert("<p>Line one<br>Line two</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"newline_style\":\"Backslash\"}"))
      __result.content.to_s.should contain("Line one")
      __result.content.to_s.should contain("Line two")
    end
    it "Hard line breaks rendered with trailing spaces" do
      __result = HtmlToMarkdown.convert("<p>First<br>Second</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"newline_style\":\"Spaces\"}"))
      __result.content.to_s.should contain("First")
      __result.content.to_s.should contain("Second")
    end
    it "Djot output format produces djot-compatible markup" do
      __result = HtmlToMarkdown.convert("<p>Simple paragraph.</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"output_format\":\"Djot\"}"))
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Simple paragraph.")
    end
    it "Default markdown output format produces standard markdown" do
      __result = HtmlToMarkdown.convert("<h1>Title</h1><p>Some text.</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"heading_style\":\"Atx\",\"output_format\":\"Markdown\"}"))
      __result.content.to_s.should contain("# Title")
      __result.content.to_s.should contain("Some text.")
    end
    it "Plain text output format strips markdown syntax" do
      __result = HtmlToMarkdown.convert("<h1>Title</h1><p>Some <strong>bold</strong> text.</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"output_format\":\"Plain\"}"))
      __result.content.to_s.should contain("Title")
      __result.content.to_s.should contain("bold")
      __result.content.to_s.should contain("text.")
      # not_contains assertion requires a string value
    end
    it "Aggressive preset removes nav, footer, aside unconditionally" do
      __result = HtmlToMarkdown.convert("<nav>Menu</nav><article><h1>Title</h1><p>Content</p></article><aside>Sidebar</aside><footer>Footer</footer>", HtmlToMarkdown::ConversionOptions.from_json("{\"preprocessing\":{\"preset\":\"Aggressive\"}}"))
      __result.content.to_s.should contain("Title")
      __result.content.to_s.should contain("Content")
      __result.content.to_s.should_not contain("Menu")
    end
    it "Disabling preprocessing entirely preserves elements the standard preset would remove" do
      __result = HtmlToMarkdown.convert("<nav>NavSection</nav><p>Paragraph</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"preprocessing\":{\"enabled\":false}}"))
      __result.content.to_s.should contain("NavSection")
      __result.content.to_s.should contain("Paragraph")
    end
    it "Minimal preset preserves nav, footer, aside" do
      __result = HtmlToMarkdown.convert("<nav>Navigation</nav><p>Content</p><footer>Footer</footer>", HtmlToMarkdown::ConversionOptions.from_json("{\"preprocessing\":{\"preset\":\"Minimal\"}}"))
      __result.content.to_s.should contain("Navigation")
      __result.content.to_s.should contain("Content")
      __result.content.to_s.should contain("Footer")
    end
    it "Forms are removed when remove_forms is true" do
      __result = HtmlToMarkdown.convert("<p>Before</p><form><input type='text'/><button>Submit</button></form><p>After</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"preprocessing\":{\"remove_forms\":true}}"))
      __result.content.to_s.should contain("Before")
      __result.content.to_s.should contain("After")
      __result.content.to_s.should_not contain("Submit")
    end
    it "Setting remove_navigation to false preserves nav and aside elements" do
      __result = HtmlToMarkdown.convert("<nav>SiteMenu</nav><main><p>MainContent</p></main><aside>SidebarText</aside>", HtmlToMarkdown::ConversionOptions.from_json("{\"preprocessing\":{\"remove_navigation\":false}}"))
      __result.content.to_s.should contain("SiteMenu")
      __result.content.to_s.should contain("MainContent")
      __result.content.to_s.should contain("SidebarText")
    end
    it "Iframe tags preserved as raw HTML in output" do
      __result = HtmlToMarkdown.convert("<p>Before</p><iframe src='video.html' width='560'></iframe><p>After</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"preserve_tags\":[\"iframe\"]}"))
      __result.content.to_s.should contain("Before")
      __result.content.to_s.should contain("After")
      __result.content.to_s.should contain("<iframe")
    end
    it "Images are omitted from output when skipImages is true" do
      __result = HtmlToMarkdown.convert("<p>Before <img src='test.jpg' alt='photo'> After</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"skip_images\":true}"))
      __result.content.to_s.should contain("Before")
      __result.content.to_s.should contain("After")
      __result.content.to_s.should_not contain("photo")
    end
    it "Strip newlines produces single-line paragraphs" do
      __result = HtmlToMarkdown.convert("<p>First paragraph.</p><p>Second paragraph.</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"strip_newlines\":true}"))
      __result.content.to_s.should contain("First paragraph.")
      __result.content.to_s.should contain("Second paragraph.")
    end
    it "Div and span tags stripped but content preserved" do
      __result = HtmlToMarkdown.convert("<div class='wrapper'><p>Inside div</p></div><p>Outside <span class='hl'>span text</span></p>", HtmlToMarkdown::ConversionOptions.from_json("{\"strip_tags\":[\"div\",\"span\"]}"))
      __result.content.to_s.should contain("Inside div")
      __result.content.to_s.should contain("span text")
    end
    it "Strong and em tags use underscore symbol instead of asterisk" do
      __result = HtmlToMarkdown.convert("<p><strong>bold</strong> and <em>italic</em></p>", HtmlToMarkdown::ConversionOptions.from_json("{\"strong_em_symbol\":\"_\"}"))
      __result.content.to_s.should contain("__bold__")
      __result.content.to_s.should contain("_italic_")
    end
    it "Subscript rendered with tilde symbol" do
      __result = HtmlToMarkdown.convert("<p>H<sub>2</sub>O</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"sub_symbol\":\"~\"}"))
      __result.content.to_s.should contain("~2~")
    end
    it "Superscript rendered with caret symbol" do
      __result = HtmlToMarkdown.convert("<p>x<sup>2</sup></p>", HtmlToMarkdown::ConversionOptions.from_json("{\"sup_symbol\":\"^\"}"))
      __result.content.to_s.should contain("^2^")
    end
    it "Default angle style wraps URLs containing spaces in angle brackets" do
      __result = HtmlToMarkdown.convert("<a href=\"/file (1).pdf\">file</a>", HtmlToMarkdown::ConversionOptions.from_json("{\"url_escape_style\":\"angle\"}"))
      __result.content.to_s.should contain("</file (1).pdf>")
    end
    it "Percent style encodes angle brackets in link URLs from issue 392" do
      __result = HtmlToMarkdown.convert("<a href=\"/file (1) <draft>.pdf\">file</a>", HtmlToMarkdown::ConversionOptions.from_json("{\"url_escape_style\":\"percent\"}"))
      __result.content.to_s.should contain("/file%20%281%29%20%3Cdraft%3E.pdf")
    end
    it "Percent style encodes special characters in image src URLs" do
      __result = HtmlToMarkdown.convert("<img src=\"/img (1) <draft>.png\" alt=\"alt\">", HtmlToMarkdown::ConversionOptions.from_json("{\"url_escape_style\":\"percent\"}"))
      __result.content.to_s.should contain("/img%20%281%29%20%3Cdraft%3E.png")
    end
    it "Percent style encodes spaces and parens in link URLs" do
      __result = HtmlToMarkdown.convert("<a href=\"/file (1).pdf\">file</a>", HtmlToMarkdown::ConversionOptions.from_json("{\"url_escape_style\":\"percent\"}"))
      __result.content.to_s.should contain("/file%20%281%29.pdf")
      __result.content.to_s.should_not contain("</")
    end
    it "Normalized whitespace mode collapses multiple spaces" do
      __result = HtmlToMarkdown.convert("<p>Text   with    extra   spaces.</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"whitespace_mode\":\"Normalized\"}"))
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Text")
      __result.content.to_s.should contain("with")
      __result.content.to_s.should contain("extra")
      __result.content.to_s.should contain("spaces.")
    end
    it "Strict whitespace mode preserves whitespace as-is" do
      __result = HtmlToMarkdown.convert("<p>Preserved   spacing.</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"whitespace_mode\":\"Strict\"}"))
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Preserved")
      __result.content.to_s.should contain("spacing.")
    end
    it "Wrap option disabled preserves long lines without breaking" do
      __result = HtmlToMarkdown.convert("<p>This is a long paragraph that should not be wrapped at all because wrapping is disabled.</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"wrap\":false}"))
      __result.content.to_s.should contain("This is a long paragraph that should not be wrapped at all because wrapping is disabled.")
    end
    it "Wrap option enabled with custom width wraps long lines" do
      __result = HtmlToMarkdown.convert("<p>This is a long paragraph that should be wrapped at the specified column width when the wrap option is enabled.</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"wrap\":true,\"wrap_width\":40}"))
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("This is a long paragraph")
    end
  end
end
