require "./spec_helper"

describe HtmlToMarkdown do
  describe "result" do
    it "Result tables array is empty when input has no tables" do
      __result = HtmlToMarkdown.convert("<p>No tables here</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"include_document_structure\":true}"))
      __result.content.to_s.should_not be_empty
      __result.tables.size.should eq(0)
    end
    it "Multiple tables each appear in the tables array" do
      __result = HtmlToMarkdown.convert("<table><tr><th>A</th></tr><tr><td>1</td></tr></table><p>Between</p><table><tr><th>B</th></tr><tr><td>2</td></tr></table>", HtmlToMarkdown::ConversionOptions.from_json("{\"include_document_structure\":true}"))
      __result.tables.size.should be >=(2)
    end
    it "Simple table populates the tables array in result" do
      __result = HtmlToMarkdown.convert("<table><thead><tr><th>Name</th><th>Age</th></tr></thead><tbody><tr><td>Alice</td><td>30</td></tr></tbody></table>", HtmlToMarkdown::ConversionOptions.from_json("{\"include_document_structure\":true}"))
      __result.content.to_s.should_not be_empty
      __result.tables.size.should be >=(1)
    end
    it "Tables array is empty when includeDocumentStructure is false" do
      __result = HtmlToMarkdown.convert("<table><tr><th>X</th></tr><tr><td>Y</td></tr></table>", nil)
      __result.content.to_s.should_not be_empty
      __result.tables.size.should eq(0)
    end
    it "A malformed data URI with extract_images enabled produces an ImageExtractionFailed warning" do
      __result = HtmlToMarkdown.convert("<p>Text<img src=\"data:BADMIME\" alt=\"broken\">end</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"extract_images\":true}"))
      __result.content.to_s.should_not be_empty
      __result.warnings.size.should be >=(1)
    end
    it "Warnings array is empty for well-formed HTML without problematic content" do
      __result = HtmlToMarkdown.convert("<h1>Title</h1><p>Clean content with <a href='https://example.com'>a link</a>.</p>", nil)
      __result.content.to_s.should_not be_empty
      __result.warnings.size.should eq(0)
    end
    it "Warnings array is empty for complex but valid HTML" do
      __result = HtmlToMarkdown.convert("<article><h1>Article</h1><p>Paragraph with <strong>bold</strong> and <em>italic</em>.</p><table><tr><th>Col</th></tr><tr><td>Val</td></tr></table><ul><li>Item 1</li><li>Item 2</li></ul></article>", nil)
      __result.content.to_s.should_not be_empty
      __result.warnings.size.should eq(0)
    end
    it "Warnings array is empty even for malformed HTML (parser is lenient)" do
      __result = HtmlToMarkdown.convert("<p>Unclosed paragraph<div>Mixed nesting</p></div>", nil)
      __result.content.to_s.should_not be_empty
      __result.warnings.size.should eq(0)
    end
  end
end
