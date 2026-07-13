require "./spec_helper"

module HtmlToMarkdown
  class TestVisitorVisitoraudiocustom < HtmlVisitorVisitor
    def visit_audio(ctx : HtmlVisitorVisitorContext, src : String?) : VisitResult
      VisitResult::Custom.new("[AUDIO: podcast.mp3]")
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitoraudioskip < HtmlVisitorVisitor
    def visit_audio(ctx : HtmlVisitorVisitorContext, src : String?) : VisitResult
      VisitResult::Skip.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorbuttoncustom < HtmlVisitorVisitor
    def visit_button(ctx : HtmlVisitorVisitorContext, text : String) : VisitResult
      VisitResult::Continue.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorbuttonskip < HtmlVisitorVisitor
    def visit_button(ctx : HtmlVisitorVisitorContext, text : String) : VisitResult
      VisitResult::Skip.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorcontinuedefault < HtmlVisitorVisitor
    def visit_strong(ctx : HtmlVisitorVisitorContext, text : String) : VisitResult
      VisitResult::Continue.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorcustomblockquote < HtmlVisitorVisitor
    def visit_blockquote(ctx : HtmlVisitorVisitorContext, content : String, depth : LibC::SizeT) : VisitResult
      VisitResult::Continue.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorcustomemphasis < HtmlVisitorVisitor
    def visit_emphasis(ctx : HtmlVisitorVisitorContext, text : String) : VisitResult
      VisitResult::Continue.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorcustomheading < HtmlVisitorVisitor
    def visit_heading(ctx : HtmlVisitorVisitorContext, level : UInt32, text : String, id : String?) : VisitResult
      VisitResult::Continue.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorcustomimage < HtmlVisitorVisitor
    def visit_image(ctx : HtmlVisitorVisitorContext, src : String, alt : String, title : String?) : VisitResult
      VisitResult::Continue.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorcustomlinkformat < HtmlVisitorVisitor
    def visit_link(ctx : HtmlVisitorVisitorContext, href : String, text : String, title : String?) : VisitResult
      VisitResult::Continue.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorcustomlinkstatic < HtmlVisitorVisitor
    def visit_link(ctx : HtmlVisitorVisitorContext, href : String, text : String, title : String?) : VisitResult
      VisitResult::Custom.new("[REDACTED LINK]")
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorcustomoutput < HtmlVisitorVisitor
    def visit_heading(ctx : HtmlVisitorVisitorContext, level : UInt32, text : String, id : String?) : VisitResult
      VisitResult::Custom.new("## REPLACED HEADING")
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitordefinitionlistcustom < HtmlVisitorVisitor
    def visit_definition_term(ctx : HtmlVisitorVisitorContext, text : String) : VisitResult
      VisitResult::Continue.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitordefinitionlistcustomformat < HtmlVisitorVisitor
    def visit_definition_description(ctx : HtmlVisitorVisitorContext, text : String) : VisitResult
      VisitResult::Continue.new
    end
    def visit_definition_term(ctx : HtmlVisitorVisitorContext, text : String) : VisitResult
      VisitResult::Continue.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitordefinitionlistskip < HtmlVisitorVisitor
    def visit_definition_description(ctx : HtmlVisitorVisitorContext, text : String) : VisitResult
      VisitResult::Skip.new
    end
    def visit_definition_term(ctx : HtmlVisitorVisitorContext, text : String) : VisitResult
      VisitResult::Skip.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitordetailssummarycustom < HtmlVisitorVisitor
    def visit_summary(ctx : HtmlVisitorVisitorContext, text : String) : VisitResult
      VisitResult::Continue.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitordetailssummaryskip < HtmlVisitorVisitor
    def visit_details(ctx : HtmlVisitorVisitorContext, open : Bool) : VisitResult
      VisitResult::Skip.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorfigurecustom < HtmlVisitorVisitor
    def visit_figcaption(ctx : HtmlVisitorVisitorContext, text : String) : VisitResult
      VisitResult::Continue.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorfigurecustomwrap < HtmlVisitorVisitor
    def visit_figure_end(ctx : HtmlVisitorVisitorContext, output : String) : VisitResult
      VisitResult::Continue.new
    end
    def visit_figure_start(ctx : HtmlVisitorVisitorContext) : VisitResult
      VisitResult::Custom.new("
[FIGURE]
")
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorfigureskip < HtmlVisitorVisitor
    def visit_figure_start(ctx : HtmlVisitorVisitorContext) : VisitResult
      VisitResult::Skip.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorformcustom < HtmlVisitorVisitor
    def visit_form(ctx : HtmlVisitorVisitorContext, action : String?, method : String?) : VisitResult
      VisitResult::Custom.new("[FORM PLACEHOLDER]")
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorformskip < HtmlVisitorVisitor
    def visit_form(ctx : HtmlVisitorVisitorContext, action : String?, method : String?) : VisitResult
      VisitResult::Skip.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorheadingbarestringpreservescase < HtmlVisitorVisitor
    def visit_heading(ctx : HtmlVisitorVisitorContext, level : UInt32, text : String, id : String?) : VisitResult
      VisitResult::Continue.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorhorizontalrulecustom < HtmlVisitorVisitor
    def visit_horizontal_rule(ctx : HtmlVisitorVisitorContext) : VisitResult
      VisitResult::Custom.new("
[DIVIDER]
")
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorhorizontalruleskip < HtmlVisitorVisitor
    def visit_horizontal_rule(ctx : HtmlVisitorVisitorContext) : VisitResult
      VisitResult::Skip.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitoriframecustom < HtmlVisitorVisitor
    def visit_iframe(ctx : HtmlVisitorVisitorContext, src : String?) : VisitResult
      VisitResult::Custom.new("[EMBEDDED: https://maps.example.com/embed]")
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitoriframeskip < HtmlVisitorVisitor
    def visit_iframe(ctx : HtmlVisitorVisitorContext, src : String?) : VisitResult
      VisitResult::Skip.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorimagebarestringpreservescase < HtmlVisitorVisitor
    def visit_image(ctx : HtmlVisitorVisitorContext, src : String, alt : String, title : String?) : VisitResult
      VisitResult::Continue.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorinputcustom < HtmlVisitorVisitor
    def visit_input(ctx : HtmlVisitorVisitorContext, input_type : String, name : String?, value : String?) : VisitResult
      VisitResult::Continue.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorinputskip < HtmlVisitorVisitor
    def visit_input(ctx : HtmlVisitorVisitorContext, input_type : String, name : String?, value : String?) : VisitResult
      VisitResult::Skip.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorlinebreakcustom < HtmlVisitorVisitor
    def visit_line_break(ctx : HtmlVisitorVisitorContext) : VisitResult
      VisitResult::Custom.new(" | ")
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorlinebreakskip < HtmlVisitorVisitor
    def visit_line_break(ctx : HtmlVisitorVisitorContext) : VisitResult
      VisitResult::Skip.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorlinkbarestringpreservescase < HtmlVisitorVisitor
    def visit_link(ctx : HtmlVisitorVisitorContext, href : String, text : String, title : String?) : VisitResult
      VisitResult::Continue.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitormarkcustom < HtmlVisitorVisitor
    def visit_mark(ctx : HtmlVisitorVisitorContext, text : String) : VisitResult
      VisitResult::Continue.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitormarkskip < HtmlVisitorVisitor
    def visit_mark(ctx : HtmlVisitorVisitorContext, text : String) : VisitResult
      VisitResult::Skip.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorpreservehtml < HtmlVisitorVisitor
    def visit_custom_element(ctx : HtmlVisitorVisitorContext, tag_name : String, html : String) : VisitResult
      VisitResult::PreserveHtml.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorskipcodeblocks < HtmlVisitorVisitor
    def visit_code_block(ctx : HtmlVisitorVisitorContext, lang : String?, code : String) : VisitResult
      VisitResult::Skip.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorskipheading < HtmlVisitorVisitor
    def visit_heading(ctx : HtmlVisitorVisitorContext, level : UInt32, text : String, id : String?) : VisitResult
      VisitResult::Skip.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorskipimages < HtmlVisitorVisitor
    def visit_image(ctx : HtmlVisitorVisitorContext, src : String, alt : String, title : String?) : VisitResult
      VisitResult::Skip.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorskiplinks < HtmlVisitorVisitor
    def visit_link(ctx : HtmlVisitorVisitorContext, href : String, text : String, title : String?) : VisitResult
      VisitResult::Skip.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorskipstrong < HtmlVisitorVisitor
    def visit_strong(ctx : HtmlVisitorVisitorContext, text : String) : VisitResult
      VisitResult::Skip.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorsubscriptcustom < HtmlVisitorVisitor
    def visit_subscript(ctx : HtmlVisitorVisitorContext, text : String) : VisitResult
      VisitResult::Continue.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorsubscriptskip < HtmlVisitorVisitor
    def visit_subscript(ctx : HtmlVisitorVisitorContext, text : String) : VisitResult
      VisitResult::Skip.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorsuperscriptcustom < HtmlVisitorVisitor
    def visit_superscript(ctx : HtmlVisitorVisitorContext, text : String) : VisitResult
      VisitResult::Continue.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorsuperscriptskip < HtmlVisitorVisitor
    def visit_superscript(ctx : HtmlVisitorVisitorContext, text : String) : VisitResult
      VisitResult::Skip.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorunderlinecustom < HtmlVisitorVisitor
    def visit_underline(ctx : HtmlVisitorVisitorContext, text : String) : VisitResult
      VisitResult::Continue.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorunderlineskip < HtmlVisitorVisitor
    def visit_underline(ctx : HtmlVisitorVisitorContext, text : String) : VisitResult
      VisitResult::Skip.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorvideocustom < HtmlVisitorVisitor
    def visit_video(ctx : HtmlVisitorVisitorContext, src : String?) : VisitResult
      VisitResult::Continue.new
    end
  end
end

module HtmlToMarkdown
  class TestVisitorVisitorvideoskip < HtmlVisitorVisitor
    def visit_video(ctx : HtmlVisitorVisitorContext, src : String?) : VisitResult
      VisitResult::Skip.new
    end
  end
end

describe HtmlToMarkdown do
  describe "visitor" do
    it "Visitor replaces audio element with custom output" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitoraudiocustom.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>Listen to this: <audio src=\"podcast.mp3\" controls></audio></p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("[AUDIO: podcast.mp3]")
      __result.content.to_s.should contain("Listen to this:")
    end
    it "Visitor removes audio elements from output" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitoraudioskip.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>Background music:</p><audio src=\"music.ogg\" autoplay></audio><p>Enjoy!</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("Background music:")
      __result.content.to_s.should contain("Enjoy!")
      __result.content.to_s.should_not contain("music.ogg")
    end
    it "Visitor replaces button with bracketed text" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorbuttoncustom.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>Confirm action: <button type=\"submit\">Click me</button> or <button type=\"reset\">Cancel</button></p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("[BTN:Click me]")
      __result.content.to_s.should contain("[BTN:Cancel]")
      __result.content.to_s.should contain("Confirm action:")
    end
    it "Visitor removes all buttons from output" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorbuttonskip.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>Actions available: <button>Save</button> <button>Delete</button> <button>Export</button></p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("Actions available:")
      __result.content.to_s.should_not contain("Save")
      __result.content.to_s.should_not contain("Delete")
      __result.content.to_s.should_not contain("Export")
    end
    it "Visitor continue action preserves default conversion" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorcontinuedefault.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>Hello <strong>World</strong></p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("**World**")
    end
    it "Visitor replaces blockquote with custom format" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorcustomblockquote.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<blockquote><p>A wise quote.</p></blockquote>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("QUOTE:")
      __result.content.to_s.should contain("A wise quote.")
    end
    it "Visitor replaces emphasis with custom output" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorcustomemphasis.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>This is <em>important</em> text.</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain(">>>important<<<")
      __result.content.to_s.should_not contain("*important*")
    end
    it "Visitor replaces heading with custom format" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorcustomheading.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<h2>Section Title</h2><p>Content below heading.</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("--- Section Title ---")
      __result.content.to_s.should_not contain("## Section Title")
      __result.content.to_s.should contain("Content below heading.")
    end
    it "Visitor replaces image with custom output using template" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorcustomimage.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<img src=\"banner.png\" alt=\"Banner\">", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("[Image: Banner]")
      __result.content.to_s.should_not contain("banner.png")
    end
    it "Visitor reformats links using template interpolation" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorcustomlinkformat.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>Visit <a href=\"https://example.com\">Example</a> for more info.</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("Example (https://example.com)")
      __result.content.to_s.should_not contain("[Example]")
    end
    it "Visitor replaces link with static custom output" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorcustomlinkstatic.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<a href=\"https://example.com\">Click here</a>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("[REDACTED LINK]")
      __result.content.to_s.should_not contain("example.com")
    end
    it "Visitor custom action replaces element output" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorcustomoutput.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<h1>Original Heading</h1>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("## REPLACED HEADING")
      __result.content.to_s.should_not contain("# Original Heading")
    end
    it "Visitor customizes definition list items" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitordefinitionlistcustom.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<dl><dt>HTML</dt><dd>HyperText Markup Language</dd><dt>CSS</dt><dd>Cascading Style Sheets</dd></dl>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("**HTML**")
      __result.content.to_s.should contain("**CSS**")
      __result.content.to_s.should contain("HyperText Markup Language")
    end
    it "Visitor formats definition lists with custom templates" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitordefinitionlistcustomformat.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<dl><dt>Python</dt><dd>A high-level programming language</dd><dt>JavaScript</dt><dd>A scripting language for web browsers</dd></dl>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("### Python")
      __result.content.to_s.should contain("### JavaScript")
      __result.content.to_s.should contain("> A high-level programming language")
      __result.content.to_s.should contain("> A scripting language for web browsers")
    end
    it "Visitor skips definition list items from output" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitordefinitionlistskip.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>Glossary:</p><dl><dt>Term A</dt><dd>Definition of term A</dd><dt>Term B</dt><dd>Definition of term B</dd></dl><p>End of glossary</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("Glossary:")
      __result.content.to_s.should contain("End of glossary")
      __result.content.to_s.should_not contain("Term A")
      __result.content.to_s.should_not contain("Definition")
    end
    it "Visitor customizes details/summary disclosure elements" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitordetailssummarycustom.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<details><summary>Click to expand</summary><p>This content is initially hidden.</p><p>But can be revealed by the user.</p></details>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("[EXPANDABLE] Click to expand")
      __result.content.to_s.should contain("This content is initially hidden.")
    end
    it "Visitor removes details/summary elements entirely" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitordetailssummaryskip.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>Main content here.</p><details><summary>Hidden section</summary><p>Secret details</p></details><p>More main content.</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("Main content here.")
      __result.content.to_s.should contain("More main content.")
      __result.content.to_s.should_not contain("Hidden section")
      __result.content.to_s.should_not contain("Secret details")
    end
    it "Visitor customizes figure and figcaption elements" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorfigurecustom.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<article><h1>Article Title</h1><p>Introduction paragraph.</p><figure><img src=\"diagram.png\" alt=\"System architecture diagram\"><figcaption>Figure 1: System Architecture</figcaption></figure><p>Explanation of the figure.</p></article>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("Article Title")
      __result.content.to_s.should contain("*Figure 1: System Architecture*")
      __result.content.to_s.should contain("Explanation of the figure.")
    end
    it "Visitor wraps figure content with custom formatting" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorfigurecustomwrap.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<section><h2>Gallery</h2><figure><img src=\"photo1.jpg\" alt=\"Photo\"><figcaption>Beautiful sunset</figcaption></figure></section>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("[FIGURE]")
      __result.content.to_s.should contain("[/FIGURE]")
      __result.content.to_s.should contain("Gallery")
    end
    it "Visitor removes figure elements with their captions" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorfigureskip.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>See the chart below:</p><figure><img src=\"chart.svg\"><figcaption>Revenue Trends 2020-2024</figcaption></figure><p>As shown in the chart above.</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("See the chart below:")
      __result.content.to_s.should contain("As shown in the chart above.")
      __result.content.to_s.should_not contain("Revenue Trends")
      __result.content.to_s.should_not contain("chart.svg")
    end
    it "Visitor replaces form with custom output" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorformcustom.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<div><form action=\"/submit\" method=\"POST\"><label>Name: <input type=\"text\" name=\"name\"></label><button type=\"submit\">Submit</button></form></div>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("[FORM PLACEHOLDER]")
      __result.content.to_s.should_not contain("submit")
      __result.content.to_s.should_not contain("input")
    end
    it "Visitor skips form elements entirely" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorformskip.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>Before form</p><form><input type=\"email\" name=\"email\"></form><p>After form</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("Before form")
      __result.content.to_s.should contain("After form")
      __result.content.to_s.should_not contain("email")
    end
    it "Visitor returns rendered heading template as bare string; mixed-case is preserved (regression guard for issue #350)" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorheadingbarestringpreservescase.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<h2>Important Section Title</h2><p>Body.</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("## Important Section Title ##")
      __result.content.to_s.should_not contain("important section title")
    end
    it "Visitor replaces horizontal rule with custom output" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorhorizontalrulecustom.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<h1>Section A</h1><p>Content A</p><hr><h1>Section B</h1><p>Content B</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("[DIVIDER]")
      __result.content.to_s.should contain("Section A")
      __result.content.to_s.should contain("Section B")
      __result.content.to_s.should_not contain("---")
    end
    it "Visitor removes all horizontal rules" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorhorizontalruleskip.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>Part 1</p><hr><p>Part 2</p><hr><p>Part 3</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("Part 1")
      __result.content.to_s.should contain("Part 2")
      __result.content.to_s.should contain("Part 3")
      __result.content.to_s.should_not contain("---")
    end
    it "Visitor replaces embedded iframe with custom text" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitoriframecustom.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>Embedded map:</p><iframe src=\"https://maps.example.com/embed\" width=\"400\" height=\"300\"></iframe><p>End of map</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("[EMBEDDED: https://maps.example.com/embed]")
      __result.content.to_s.should contain("Embedded map:")
      __result.content.to_s.should contain("End of map")
    end
    it "Visitor removes embedded iframes" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitoriframeskip.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<h3>Reviews</h3><iframe src=\"https://widget.example.com/reviews\"></iframe><p>See reviews from our partners.</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("Reviews")
      __result.content.to_s.should contain("See reviews from our partners.")
      __result.content.to_s.should_not contain("widget.example.com")
    end
    it "Visitor returns bare-string image replacement; mixed-case alt/src preserved (regression guard for issue #350)" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorimagebarestringpreservescase.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<img src=\"PhotoOne.JPG\" alt=\"Sunset Over Bay\">", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("[image: Sunset Over Bay -> PhotoOne.JPG]")
      __result.content.to_s.should_not contain("sunset over bay")
      __result.content.to_s.should_not contain("photoone.jpg")
    end
    it "Visitor replaces input with labeled output" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorinputcustom.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<form><label>Username: <input type=\"text\" name=\"username\" value=\"\"></label><label>Password: <input type=\"password\" name=\"password\"></label></form>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("[INPUT:text]")
      __result.content.to_s.should contain("[INPUT:password]")
    end
    it "Visitor skips all input elements" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorinputskip.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>Sign up:</p><input type=\"text\" name=\"email\" placeholder=\"your@email.com\"><input type=\"checkbox\" name=\"agree\"><p>Continue</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("Sign up:")
      __result.content.to_s.should contain("Continue")
      __result.content.to_s.should_not contain("email")
    end
    it "Visitor replaces line break with custom output" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorlinebreakcustom.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>First line<br>Second line<br>Third line</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("First line | Second line | Third line")
      __result.content.to_s.should_not contain("\n\n")
    end
    it "Visitor removes all line breaks" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorlinebreakskip.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>Address Line 1<br>Address Line 2<br>Address Line 3</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("Address Line 1Address Line 2Address Line 3")
    end
    it "Visitor returns rendered template as a bare string (not a dict) and original mixed-case content is preserved end-to-end (regression guard for issue #350)" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorlinkbarestringpreservescase.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<a href=\"https://old-cdn.com/file.pdf\">Download</a>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("[Download](https://new-cdn.com/file.pdf)")
      __result.content.to_s.should_not contain("[download]")
      __result.content.to_s.should_not contain("old-cdn.com")
    end
    it "Visitor replaces highlight/mark with custom template" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitormarkcustom.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>This is a <mark>highlighted passage</mark> in the text.</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("==highlighted passage==")
      __result.content.to_s.should contain("This is a")
    end
    it "Visitor skips mark elements entirely" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitormarkskip.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>Key insight: <mark>always validate input</mark> for security.</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should_not contain("always validate input")
      __result.content.to_s.should contain("Key insight:")
      __result.content.to_s.should contain("for security.")
    end
    it "Visitor preserve_html action includes raw HTML in output" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorpreservehtml.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<div><custom-tag>Custom content</custom-tag></div>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("<custom-tag>")
    end
    it "Visitor skips code blocks from output" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorskipcodeblocks.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>Intro text</p><pre><code>let x = 42;</code></pre><p>Outro text</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("Intro text")
      __result.content.to_s.should contain("Outro text")
      __result.content.to_s.should_not contain("let x = 42")
    end
    it "Visitor skip action omits all headings from output" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorskipheading.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<h1>Title</h1><p>Body text remains.</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should_not contain("Title")
      __result.content.to_s.should contain("Body text remains.")
    end
    it "Visitor skips all images from output" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorskipimages.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>Before image</p><img src=\"photo.jpg\" alt=\"A photo\"><p>After image</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("Before image")
      __result.content.to_s.should contain("After image")
      __result.content.to_s.should_not contain("photo.jpg")
      __result.content.to_s.should_not contain("A photo")
    end
    it "Visitor skips all links entirely" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorskiplinks.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>Before <a href=\"https://example.com\">link text</a> after</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should_not contain("link text")
      __result.content.to_s.should_not contain("example.com")
    end
    it "Visitor skips bold/strong elements" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorskipstrong.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>Normal <strong>bold text</strong> normal</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should_not contain("bold text")
      __result.content.to_s.should contain("Normal")
    end
    it "Visitor replaces subscript with custom template" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorsubscriptcustom.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>H<sub>2</sub>O is water.</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("H~2~O")
      __result.content.to_s.should contain("is water")
    end
    it "Visitor skips subscript elements entirely" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorsubscriptskip.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>The formula C<sub>12</sub>H<sub>22</sub>O<sub>11</sub> is sugar.</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("The formula CHO is sugar.")
    end
    it "Visitor replaces superscript with custom template" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorsuperscriptcustom.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>Einstein's E=mc<sup>2</sup> revolutionized physics.</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("E=mc^2^")
      __result.content.to_s.should contain("revolutionized physics")
    end
    it "Visitor skips superscript from output" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorsuperscriptskip.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>The equation x<sup>3</sup> + y<sup>3</sup> = z<sup>3</sup> has no solutions.</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("The equation x + y = z has no solutions.")
    end
    it "Visitor replaces underline with custom markup" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorunderlinecustom.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>This is <u>very important</u> text.</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("_very important_")
      __result.content.to_s.should_not contain("**")
    end
    it "Visitor skips underline elements from output" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorunderlineskip.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>Normal text with <u>underlined part</u> and more text.</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("Normal text with")
      __result.content.to_s.should contain("and more text.")
      __result.content.to_s.should_not contain("underlined part")
    end
    it "Visitor replaces video with custom link" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorvideocustom.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<p>Watch our tutorial:</p><video src=\"tutorial.mp4\" width=\"320\" height=\"240\" controls></video><p>Great content!</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("[VIDEO: tutorial.mp4]")
      __result.content.to_s.should contain("Watch our tutorial:")
      __result.content.to_s.should contain("Great content!")
    end
    it "Visitor removes video elements entirely" do
      __visitor = HtmlToMarkdown.register_html_visitor_visitor(HtmlToMarkdown::TestVisitorVisitorvideoskip.new)
      __opts = LibHtm.conversion_options_from_json("{}")
      LibHtm.html_visitor_options_set_visitor(__opts, __visitor)
      __c_ptr = LibHtm.convert("<h2>Demo</h2><video src=\"demo.webm\"></video><p>See the demo above.</p>", __opts)
      raise "convert returned null" if __c_ptr.null?
      __c_json = String.new(LibHtm.conversion_result_to_json(__c_ptr))
      LibHtm.conversion_result_free(__c_ptr)
      LibHtm.conversion_options_free(__opts)
      __result = HtmlToMarkdown::ConversionResult.from_json(__c_json)
      __result.content.to_s.should contain("Demo")
      __result.content.to_s.should contain("See the demo above.")
      __result.content.to_s.should_not contain("demo.webm")
    end
  end
end
