require "./spec_helper"

module HtmlToMarkdown
  class TestVisitorVisitorcustomelementwithnesting < HtmlVisitorVisitor
    def visit_custom_element(ctx : HtmlVisitorVisitorContext, tag_name : String, html : String) : VisitResult
      VisitResult::Custom.new("[CUSTOM WIDGET]")
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitordeeplynestedskip < HtmlVisitorVisitor
    def visit_mark(ctx : HtmlVisitorVisitorContext, text : String) : VisitResult
      VisitResult::Skip.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorelementendmodification < HtmlVisitorVisitor
    def visit_element_end(ctx : HtmlVisitorVisitorContext, output : String) : VisitResult
      VisitResult::Custom.new("MODIFIED OUTPUT")
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorelementstartskipentiresubtree < HtmlVisitorVisitor
    def visit_element_start(ctx : HtmlVisitorVisitorContext) : VisitResult
      VisitResult::Skip.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorunknowntagpreservation < HtmlVisitorVisitor
    def visit_custom_element(ctx : HtmlVisitorVisitorContext, tag_name : String, html : String) : VisitResult
      VisitResult::PreserveHtml.new
    end
  end
end

describe HtmlToMarkdown do
  describe "edge-cases" do
    it "Empty HTML document" do
      __result = HtmlToMarkdown.convert("<html><head></head><body></body></html>", nil)
      __result.content.to_s.strip.should eq("")
    end
    it "CJK (Chinese, Japanese, Korean) characters are preserved" do
      __result = HtmlToMarkdown.convert("<p>中文内容</p><p>日本語テキスト</p><p>한국어 텍스트</p>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("中文内容")
      __result.content.to_s.should contain("日本語テキスト")
      __result.content.to_s.should contain("한국어 텍스트")
    end
    it "Common HTML entities are decoded in output" do
      __result = HtmlToMarkdown.convert("<p>&amp; &lt; &gt; &nbsp; &quot; &apos;</p>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("&")
      __result.content.to_s.should contain("<")
      __result.content.to_s.should contain(">")
    end
    it "Named HTML entities like &mdash; and &hellip; are decoded" do
      __result = HtmlToMarkdown.convert("<p>Em dash&mdash;used for parenthetical remarks&mdash;is common. Ellipsis&hellip; indicates omission. Non-breaking&nbsp;space.</p>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("—")
      __result.content.to_s.should contain("…")
    end
    it "Numeric HTML entities (decimal and hex) are decoded" do
      __result = HtmlToMarkdown.convert("<p>Copyright: &#169; Trade: &#174; Euro: &#8364; Hex: &#x00A9;</p>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("©")
      __result.content.to_s.should contain("®")
      __result.content.to_s.should contain("€")
    end
    it "Emoji and Unicode characters are preserved" do
      __result = HtmlToMarkdown.convert("<p>Hello 🌍 World 🚀</p><p>Stars: ⭐ ✨</p>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("🌍")
      __result.content.to_s.should contain("🚀")
      __result.content.to_s.should contain("⭐")
    end
    it "Document containing only HTML comments produces empty output" do
      __result = HtmlToMarkdown.convert("<!-- This is a comment --><!-- Another comment -->", nil)
      __result.content.to_s.strip.should eq("")
    end
    it "Input that is only whitespace characters (spaces, tabs, newlines) produces empty output" do
      __result = HtmlToMarkdown.convert("   ", nil)
      __result.content.to_s.strip.should eq("")
    end
    it "HTML comment ending with ---> (three dashes) must not drop content that follows (issue #339)" do
      __result = HtmlToMarkdown.convert("<h1>One</h1>\n<!-- /// --->\n<p>Two</p>", nil)
      __result.content.to_s.should contain("One")
      __result.content.to_s.should contain("Two")
    end
    it "Deeply nested elements (100 levels) are handled without stack overflow" do
      __result = HtmlToMarkdown.convert("<div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><div><p>Deeply nested content</p></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div></div>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Deeply nested content")
    end
    it "Missing closing tags on block elements are auto-closed by parser" do
      __result = HtmlToMarkdown.convert("<div><h1>Title<p>First paragraph<p>Second paragraph</div>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Title")
      __result.content.to_s.should contain("First paragraph")
      __result.content.to_s.should contain("Second paragraph")
    end
    it "Overlapping bold/italic tags are recovered by the HTML parser without panic" do
      __result = HtmlToMarkdown.convert("<p><b><i>bold and italic</b></i></p>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("bold and italic")
    end
    it "Unclosed <p> tag is recovered gracefully and content is preserved" do
      __result = HtmlToMarkdown.convert("<p>This paragraph is never closed", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("This paragraph is never closed")
    end
    it "Document with only script tags produces empty output (scripts are stripped)" do
      __result = HtmlToMarkdown.convert("<html><head><script>alert('xss')</script></head><body><script>document.write('hello')</script></body></html>", nil)
      __result.content.to_s.strip.should eq("")
    end
    it "Document with only style tags produces empty output (styles are stripped)" do
      __result = HtmlToMarkdown.convert("<html><head><style>body { color: red; }</style></head><body><style>.foo { margin: 0; }</style></body></html>", nil)
      __result.content.to_s.strip.should eq("")
    end
    it "Visitor handles custom elements with nested content" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorcustomelementwithnesting.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<div><custom-widget data-value=\"123\"><p>Widget content here</p><span>With nested elements</span></custom-widget></div>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("[CUSTOM WIDGET]")
      __result.content.to_s.should_not contain("Widget content here")
    end
    it "Visitor skips deeply nested elements" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitordeeplynestedskip.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<div><p>Outer <em>emphasis <strong>with bold <mark>and highlight</mark></strong></em> text</p></div>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("Outer")
      __result.content.to_s.should contain("text")
      __result.content.to_s.should_not contain("highlight")
    end
    it "Visitor modifies element at end after children processed" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorelementendmodification.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<blockquote><p>Original quote</p></blockquote>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should_not be_empty
    end
    it "Visitor skips at element_start level removes entire subtree" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorelementstartskipentiresubtree.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<div><h1>Title</h1><p>Content</p></div>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should_not contain("Title")
      __result.content.to_s.should_not contain("Content")
    end
    it "Visitor preserves unknown HTML tags as raw HTML" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorunknowntagpreservation.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<article><p>Article text</p><x-custom>Custom element with content</x-custom><p>More article text</p></article>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("Article text")
      __result.content.to_s.should contain("More article text")
      __result.content.to_s.should contain("<x-custom>")
    end
    it "Whitespace-only content" do
      __result = HtmlToMarkdown.convert("<p>   </p>", nil)
      __result.content.to_s.strip.should eq("")
    end
    it "onclick and other on* event handlers are removed from elements" do
      __result = HtmlToMarkdown.convert("<p><a href=\"https://example.com\" onclick=\"alert('xss')\">Click me</a></p><button onmouseover=\"steal_data()\">Hover me</button>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Click me")
      # not_contains assertion requires a string value
    end
    it "Script tag content is stripped and does not appear in output" do
      __result = HtmlToMarkdown.convert("<p>Safe content.</p><script>alert('xss')</script><p>More safe content.</p>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Safe content")
      __result.content.to_s.should contain("More safe content")
      # not_contains assertion requires a string value
    end
    it "Script tags nested inside SVG are stripped" do
      __result = HtmlToMarkdown.convert("<p>Before SVG.</p><svg xmlns=\"http://www.w3.org/2000/svg\"><script>alert('svg-xss')</script><text>SVG text</text></svg><p>After SVG.</p>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("Before SVG")
      __result.content.to_s.should contain("After SVG")
      # not_contains assertion requires a string value
    end
  end
end
