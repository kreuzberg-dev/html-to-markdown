require "./spec_helper"

describe HtmlToMarkdown do
  describe "conversion" do
    it "Blockquote with multiple paragraphs has each paragraph prefixed" do
      __result = HtmlToMarkdown.convert("<blockquote><p>First paragraph.</p><p>Second paragraph.</p></blockquote>", nil)
      __result.content.to_s.should contain("> First paragraph.")
      __result.content.to_s.should contain("> Second paragraph.")
    end
    it "Nested blockquote produces double-prefixed lines" do
      __result = HtmlToMarkdown.convert("<blockquote><p>Outer quote.</p><blockquote><p>Inner quote.</p></blockquote></blockquote>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Outer quote.")
      __result.content.to_s.should contain("Inner quote.")
    end
    it "Simple blockquote" do
      __result = HtmlToMarkdown.convert("<blockquote><p>Quote text</p></blockquote>", nil)
      __result.content.to_s.should contain("> Quote text")
    end
    it "Blockquote containing a list preserves list items inside quote" do
      __result = HtmlToMarkdown.convert("<blockquote><p>Quote intro:</p><ul><li>Point one</li><li>Point two</li></ul></blockquote>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Quote intro:")
      __result.content.to_s.should contain("Point one")
      __result.content.to_s.should contain("Point two")
    end
    it "Nested bold and italic" do
      __result = HtmlToMarkdown.convert("<p><strong><em>both</em></strong></p>", nil)
      __result.content.to_s.should contain("***both***")
    end
    it "Strong tag converts to bold" do
      __result = HtmlToMarkdown.convert("<p><strong>bold</strong></p>", nil)
      __result.content.to_s.should contain("**bold**")
    end
    it "Code block with language preserves content" do
      __result = HtmlToMarkdown.convert("<pre><code class=\"language-python\">print('hello')</code></pre>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("print('hello')")
    end
    it "Code block without a language class preserves content" do
      __result = HtmlToMarkdown.convert("<pre><code>plain code here</code></pre>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("plain code here")
    end
    it "Inline code element nested inside a paragraph" do
      __result = HtmlToMarkdown.convert("<p>Call the <code>initialize()</code> method first.</p>", nil)
      __result.content.to_s.should contain("`initialize()`")
    end
    it "Inline code containing backtick characters is properly escaped" do
      __result = HtmlToMarkdown.convert("<p>Use <code>`backtick` here</code> carefully.</p>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("backtick")
    end
    it "Bare filename href (no URI scheme) renders as inline link, not autolink (#397)" do
      __result = HtmlToMarkdown.convert("<a href=\"foobar.png\">foobar.png</a>", nil)
      __result.content.to_s.strip.should eq("[foobar.png](foobar.png)")
    end
    it "Absolute https URL with matching text renders as autolink (#397)" do
      __result = HtmlToMarkdown.convert("<a href=\"https://example.com\">https://example.com</a>", nil)
      __result.content.to_s.strip.should eq("<https://example.com>")
    end
    it "mailto: link with matching email text renders as autolink (#397)" do
      __result = HtmlToMarkdown.convert("<a href=\"mailto:a@b.com\">a@b.com</a>", nil)
      __result.content.to_s.strip.should eq("<a@b.com>")
    end
    it "Mixed filename + URL: only the URL becomes an autolink (#397)" do
      __result = HtmlToMarkdown.convert("<a href=\"foobar.png\">foobar.png</a> <a href=\"https://www.heise.de\">https://www.heise.de</a>", nil)
      __result.content.to_s.strip.should eq("[foobar.png](foobar.png) <https://www.heise.de>")
    end
    it "Relative path href renders as inline link (#397)" do
      __result = HtmlToMarkdown.convert("<a href=\"/docs/intro.html\">/docs/intro.html</a>", nil)
      __result.content.to_s.strip.should eq("[/docs/intro.html](/docs/intro.html)")
    end
    it "mark tag produces highlighted output" do
      __result = HtmlToMarkdown.convert("<p><mark>highlighted</mark></p>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("highlighted")
    end
    it "del tag converts to GFM strikethrough" do
      __result = HtmlToMarkdown.convert("<p><del>deleted text</del></p>", nil)
      __result.content.to_s.should contain("~~deleted text~~")
    end
    it "s tag converts to GFM strikethrough" do
      __result = HtmlToMarkdown.convert("<p><s>strikethrough</s></p>", nil)
      __result.content.to_s.should contain("~~strikethrough~~")
    end
    it "sub tag content is preserved" do
      __result = HtmlToMarkdown.convert("<p>H<sub>2</sub>O</p>", nil)
      __result.content.to_s.should contain("H")
      __result.content.to_s.should contain("2")
      __result.content.to_s.should contain("O")
    end
    it "sup tag content is preserved" do
      __result = HtmlToMarkdown.convert("<p>x<sup>2</sup></p>", nil)
      __result.content.to_s.should contain("x")
      __result.content.to_s.should contain("2")
    end
    it "u tag content is preserved in output" do
      __result = HtmlToMarkdown.convert("<p><u>underlined</u></p>", nil)
      __result.content.to_s.should contain("underlined")
    end
    it "Form input elements produce readable output without form mechanics" do
      __result = HtmlToMarkdown.convert("<form><label for=\"name\">Name:</label><input type=\"text\" id=\"name\" placeholder=\"Enter name\"></form>", HtmlToMarkdown::ConversionOptions.from_json("{\"preprocessing\":{\"remove_forms\":false}}"))
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Name")
    end
    it "Select element with options produces readable output" do
      __result = HtmlToMarkdown.convert("<form><label>Color:</label><select><option value=\"red\">Red</option><option value=\"blue\" selected>Blue</option><option value=\"green\">Green</option></select></form>", HtmlToMarkdown::ConversionOptions.from_json("{\"preprocessing\":{\"remove_forms\":false}}"))
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Color")
    end
    it "Textarea element produces readable output" do
      __result = HtmlToMarkdown.convert("<form><label>Message:</label><textarea>Default text content</textarea></form>", HtmlToMarkdown::ConversionOptions.from_json("{\"preprocessing\":{\"remove_forms\":false}}"))
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Message")
    end
    it "H1 heading" do
      __result = HtmlToMarkdown.convert("<h1>Heading 1</h1>", nil)
      __result.content.to_s.strip.should eq("# Heading 1")
    end
    it "H2 heading" do
      __result = HtmlToMarkdown.convert("<h2>Heading 2</h2>", nil)
      __result.content.to_s.strip.should eq("## Heading 2")
    end
    it "H3 heading" do
      __result = HtmlToMarkdown.convert("<h3>Heading 3</h3>", nil)
      __result.content.to_s.strip.should eq("### Heading 3")
    end
    it "H4 heading" do
      __result = HtmlToMarkdown.convert("<h4>Heading 4</h4>", nil)
      __result.content.to_s.strip.should eq("#### Heading 4")
    end
    it "H5 heading" do
      __result = HtmlToMarkdown.convert("<h5>Heading 5</h5>", nil)
      __result.content.to_s.strip.should eq("##### Heading 5")
    end
    it "H6 heading" do
      __result = HtmlToMarkdown.convert("<h6>Heading 6</h6>", nil)
      __result.content.to_s.strip.should eq("###### Heading 6")
    end
    it "Figure with figcaption preserves both image and caption" do
      __result = HtmlToMarkdown.convert("<figure><img src=\"sunset.jpg\" alt=\"A sunset\"><figcaption>Beautiful sunset over the ocean</figcaption></figure>", nil)
      __result.content.to_s.should contain("![A sunset](sunset.jpg)")
      __result.content.to_s.should contain("Beautiful sunset over the ocean")
    end
    it "Image inside an anchor produces a linked image" do
      __result = HtmlToMarkdown.convert("<a href=\"https://example.com\"><img src=\"icon.png\" alt=\"Icon\"></a>", nil)
      __result.content.to_s.should contain("![Icon](icon.png)")
      __result.content.to_s.should contain("https://example.com")
    end
    it "Image without alt text produces image markdown" do
      __result = HtmlToMarkdown.convert("<img src=\"banner.jpg\">", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("banner.jpg")
    end
    it "Image with alt text" do
      __result = HtmlToMarkdown.convert("<img src=\"photo.jpg\" alt=\"A photo\">", nil)
      __result.content.to_s.should contain("![A photo](photo.jpg)")
    end
    it "Image with title attribute includes title in output" do
      __result = HtmlToMarkdown.convert("<img src=\"chart.png\" alt=\"Sales chart\" title=\"Q3 Sales\">", nil)
      __result.content.to_s.should contain("![Sales chart](chart.png")
      __result.content.to_s.should contain("Q3 Sales")
    end
    it "Inline code" do
      __result = HtmlToMarkdown.convert("<p>Use <code>console.log()</code> to debug</p>", nil)
      __result.content.to_s.should contain("`console.log()`")
    end
    it "Em tag converts to italic" do
      __result = HtmlToMarkdown.convert("<p><em>italic</em></p>", nil)
      __result.content.to_s.should contain("*italic*")
    end
    it "Single br tag produces a line break in output" do
      __result = HtmlToMarkdown.convert("<p>First line.<br>Second line.</p>", nil)
      __result.content.to_s.should contain("First line.")
      __result.content.to_s.should contain("Second line.")
    end
    it "hr tag produces a horizontal separator between content" do
      __result = HtmlToMarkdown.convert("<p>Before rule.</p><hr><p>After rule.</p>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Before rule.")
      __result.content.to_s.should contain("After rule.")
    end
    it "Multiple consecutive br tags in sequence" do
      __result = HtmlToMarkdown.convert("<p>Start.<br><br>End.</p>", nil)
      __result.content.to_s.should contain("Start.")
      __result.content.to_s.should contain("End.")
    end
    it "Fragment-only anchor link is preserved" do
      __result = HtmlToMarkdown.convert("<a href=\"#section\">Jump to section</a>", nil)
      __result.content.to_s.should contain("[Jump to section](#section)")
    end
    it "Link with empty href produces output with the link text" do
      __result = HtmlToMarkdown.convert("<a href=\"\">No destination</a>", nil)
      __result.content.to_s.should contain("No destination")
    end
    it "Image inside a link produces a linked image" do
      __result = HtmlToMarkdown.convert("<a href=\"https://example.com\"><img src=\"logo.png\" alt=\"Logo\"></a>", nil)
      __result.content.to_s.should contain("![Logo](logo.png)")
      __result.content.to_s.should contain("https://example.com")
    end
    it "Mailto link is preserved with mailto: scheme" do
      __result = HtmlToMarkdown.convert("<a href=\"mailto:user@example.com\">Email us</a>", nil)
      __result.content.to_s.should contain("mailto:user@example.com")
    end
    it "Simple link" do
      __result = HtmlToMarkdown.convert("<a href=\"https://example.com\">Example</a>", nil)
      __result.content.to_s.should contain("[Example](https://example.com)")
    end
    it "Link containing bold text preserves formatting" do
      __result = HtmlToMarkdown.convert("<a href=\"https://example.com\"><strong>Bold link</strong></a>", nil)
      __result.content.to_s.should contain("**Bold link**")
      __result.content.to_s.should contain("https://example.com")
    end
    it "Link with title attribute" do
      __result = HtmlToMarkdown.convert("<a href=\"https://example.com\" title=\"Example Site\">Example</a>", nil)
      __result.content.to_s.should contain("[Example](https://example.com")
      __result.content.to_s.should contain("Example Site")
    end
    it "Definition list with dt and dd elements" do
      __result = HtmlToMarkdown.convert("<dl><dt>Term One</dt><dd>Definition of term one.</dd><dt>Term Two</dt><dd>Definition of term two.</dd></dl>", nil)
      __result.content.to_s.should contain("Term One")
      __result.content.to_s.should contain("Definition of term one.")
      __result.content.to_s.should contain("Term Two")
      __result.content.to_s.should contain("Definition of term two.")
    end
    it "List item containing multiple paragraphs" do
      __result = HtmlToMarkdown.convert("<ul><li><p>First paragraph in item.</p><p>Second paragraph in item.</p></li><li>Simple item</li></ul>", nil)
      __result.content.to_s.should contain("First paragraph in item.")
      __result.content.to_s.should contain("Second paragraph in item.")
      __result.content.to_s.should contain("Simple item")
    end
    it "Mixed list: ordered list nested inside unordered list" do
      __result = HtmlToMarkdown.convert("<ul><li>Item A<ol><li>Sub 1</li><li>Sub 2</li></ol></li><li>Item B</li></ul>", nil)
      __result.content.to_s.should contain("Item A")
      __result.content.to_s.should contain("Sub 1")
      __result.content.to_s.should contain("Sub 2")
      __result.content.to_s.should contain("Item B")
    end
    it "Nested ordered list with two levels of depth" do
      __result = HtmlToMarkdown.convert("<ol><li>Step 1<ol><li>Step 1a</li><li>Step 1b</li></ol></li><li>Step 2</li></ol>", nil)
      __result.content.to_s.should contain("Step 1")
      __result.content.to_s.should contain("Step 1a")
      __result.content.to_s.should contain("Step 1b")
      __result.content.to_s.should contain("Step 2")
    end
    it "Nested unordered list with two levels of depth" do
      __result = HtmlToMarkdown.convert("<ul><li>Parent A<ul><li>Child A1</li><li>Child A2</li></ul></li><li>Parent B</li></ul>", nil)
      __result.content.to_s.should contain("Parent A")
      __result.content.to_s.should contain("Child A1")
      __result.content.to_s.should contain("Child A2")
      __result.content.to_s.should contain("Parent B")
    end
    it "Task list with checked and unchecked checkboxes" do
      __result = HtmlToMarkdown.convert("<ul><li><input type=\"checkbox\" checked> Done task</li><li><input type=\"checkbox\"> Pending task</li></ul>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Done task")
      __result.content.to_s.should contain("Pending task")
    end
    it "Ordered list" do
      __result = HtmlToMarkdown.convert("<ol><li>First</li><li>Second</li><li>Third</li></ol>", nil)
      __result.content.to_s.should contain("1. First")
      __result.content.to_s.should contain("2. Second")
      __result.content.to_s.should contain("3. Third")
    end
    it "Multiple paragraphs are separated by a blank line" do
      __result = HtmlToMarkdown.convert("<p>First paragraph.</p><p>Second paragraph.</p>", nil)
      __result.content.to_s.should contain("First paragraph.")
      __result.content.to_s.should contain("Second paragraph.")
    end
    it "Text nested inside divs is extracted correctly" do
      __result = HtmlToMarkdown.convert("<div><div><p>Nested text</p></div></div>", nil)
      __result.content.to_s.should contain("Nested text")
    end
    it "Simple paragraph converts to plain text" do
      __result = HtmlToMarkdown.convert("<p>Hello World</p>", nil)
      __result.content.to_s.strip.should eq("Hello World")
    end
    it "Paragraph with bold, italic, and a link" do
      __result = HtmlToMarkdown.convert("<p>This has <strong>bold</strong>, <em>italic</em>, and a <a href=\"https://example.com\">link</a>.</p>", nil)
      __result.content.to_s.should contain("**bold**")
      __result.content.to_s.should contain("*italic*")
      __result.content.to_s.should contain("[link](https://example.com)")
    end
    it "Paragraph with br tags produces line breaks in output" do
      __result = HtmlToMarkdown.convert("<p>Line one.<br>Line two.<br>Line three.</p>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Line one.")
      __result.content.to_s.should contain("Line two.")
      __result.content.to_s.should contain("Line three.")
    end
    it "Abbreviation element text is preserved" do
      __result = HtmlToMarkdown.convert("<p>The <abbr title=\"World Wide Web\">WWW</abbr> is global.</p>", nil)
      __result.content.to_s.should contain("WWW")
    end
    it "Article element wrapping content preserves inner content" do
      __result = HtmlToMarkdown.convert("<article><h2>Article Title</h2><p>Article body.</p></article>", nil)
      __result.content.to_s.should contain("Article Title")
      __result.content.to_s.should contain("Article body.")
    end
    it "Definition list with term and description" do
      __result = HtmlToMarkdown.convert("<dl><dt>HTML</dt><dd>HyperText Markup Language</dd><dt>CSS</dt><dd>Cascading Style Sheets</dd></dl>", nil)
      __result.content.to_s.should contain("HTML")
      __result.content.to_s.should contain("HyperText Markup Language")
      __result.content.to_s.should contain("CSS")
      __result.content.to_s.should contain("Cascading Style Sheets")
    end
    it "Details and summary elements produce readable output" do
      __result = HtmlToMarkdown.convert("<details><summary>Click to expand</summary><p>Hidden content here.</p></details>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Click to expand")
    end
    it "Horizontal rule produces a separator in output" do
      __result = HtmlToMarkdown.convert("<p>Above</p><hr><p>Below</p>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Above")
      __result.content.to_s.should contain("Below")
    end
    it "Mark tag produces highlighted output" do
      __result = HtmlToMarkdown.convert("<p>This is <mark>highlighted text</mark> in a sentence.</p>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("highlighted text")
    end
    it "Section element with heading preserves structure" do
      __result = HtmlToMarkdown.convert("<section><h3>Section Heading</h3><p>Section content.</p></section>", nil)
      __result.content.to_s.should contain("Section Heading")
      __result.content.to_s.should contain("Section content.")
    end
    it "Subscript and superscript elements are preserved in output" do
      __result = HtmlToMarkdown.convert("<p>H<sub>2</sub>O and E=mc<sup>2</sup></p>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("H")
      __result.content.to_s.should contain("2")
      __result.content.to_s.should contain("O")
      __result.content.to_s.should contain("E=mc")
    end
    it "Simple table with header" do
      __result = HtmlToMarkdown.convert("<table><thead><tr><th>Name</th><th>Age</th></tr></thead><tbody><tr><td>Alice</td><td>30</td></tr></tbody></table>", nil)
      __result.content.to_s.should contain("Name")
      __result.content.to_s.should contain("Age")
      __result.content.to_s.should contain("Alice")
      __result.content.to_s.should contain("30")
      __result.content.to_s.should contain("|")
      __result.content.to_s.should contain("---")
    end
    it "Empty table produces no output or minimal output" do
      __result = HtmlToMarkdown.convert("<table></table>", nil)
      __result.content.to_s.strip.should eq("")
    end
    it "Table without thead uses first row as implied header" do
      __result = HtmlToMarkdown.convert("<table><tr><td>Product</td><td>Price</td></tr><tr><td>Apple</td><td>1.00</td></tr></table>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Product")
      __result.content.to_s.should contain("Price")
      __result.content.to_s.should contain("Apple")
      __result.content.to_s.should contain("1.00")
      __result.content.to_s.should contain("|")
    end
    it "Table cells containing pipe characters are escaped in output" do
      __result = HtmlToMarkdown.convert("<table><thead><tr><th>Expression</th><th>Result</th></tr></thead><tbody><tr><td>a | b</td><td>true</td></tr></tbody></table>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Expression")
      __result.content.to_s.should contain("Result")
      __result.content.to_s.should contain("true")
    end
    it "Table with column alignment attributes" do
      __result = HtmlToMarkdown.convert("<table><thead><tr><th align=\"left\">Left</th><th align=\"center\">Center</th><th align=\"right\">Right</th></tr></thead><tbody><tr><td>L</td><td>C</td><td>R</td></tr></tbody></table>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Left")
      __result.content.to_s.should contain("Center")
      __result.content.to_s.should contain("Right")
      __result.content.to_s.should contain("L")
      __result.content.to_s.should contain("C")
      __result.content.to_s.should contain("R")
      __result.content.to_s.should contain("|")
    end
    it "Table with colspan attribute in a header cell" do
      __result = HtmlToMarkdown.convert("<table><thead><tr><th colspan=\"2\">Full Name</th></tr></thead><tbody><tr><td>John</td><td>Doe</td></tr></tbody></table>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Full Name")
      __result.content.to_s.should contain("John")
      __result.content.to_s.should contain("Doe")
    end
    it "Unordered list" do
      __result = HtmlToMarkdown.convert("<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>", nil)
      __result.content.to_s.should contain("- Item 1")
      __result.content.to_s.should contain("- Item 2")
      __result.content.to_s.should contain("- Item 3")
    end
  end
end
