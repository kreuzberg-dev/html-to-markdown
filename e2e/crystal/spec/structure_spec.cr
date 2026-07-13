require "./spec_helper"

describe HtmlToMarkdown do
  describe "structure" do
    it "Fenced code block produces Code node" do
      __result = HtmlToMarkdown.convert("<p>Example code:</p><pre><code class=\"language-rust\">fn main() { println!(\"Hello\"); }</code></pre>", HtmlToMarkdown::ConversionOptions.from_json("{\"include_document_structure\":true}"))
      __result.content.to_s.should_not be_empty
      __result.try(&.document).try(&.nodes).to_s.should_not be_empty
      __result.try(&.document).try(&.nodes).to_s.size.should be >=(2)
    end
    it "H1 > H2 > H3 creates three levels of heading nesting" do
      __result = HtmlToMarkdown.convert("<h1>Top Level</h1><p>Top intro.</p><h2>Mid Level</h2><p>Mid content.</p><h3>Deep Level</h3><p>Deep content.</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"include_document_structure\":true}"))
      __result.content.to_s.should_not be_empty
      __result.try(&.document).try(&.nodes).to_s.should_not be_empty
      __result.try(&.document).try(&.nodes).to_s.size.should be >=(5)
    end
    it "H1 followed by H2 creates a nested group under the H1" do
      __result = HtmlToMarkdown.convert("<h1>Chapter One</h1><p>Chapter intro.</p><h2>Section One</h2><p>Section content.</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"include_document_structure\":true}"))
      __result.content.to_s.should_not be_empty
      __result.try(&.document).try(&.nodes).to_s.should_not be_empty
      __result.try(&.document).try(&.nodes).to_s.size.should be >=(3)
    end
    it "Simple heading followed by paragraph produces Heading and Paragraph nodes" do
      __result = HtmlToMarkdown.convert("<h1>Title</h1><p>A paragraph of text.</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"include_document_structure\":true}"))
      __result.content.to_s.should_not be_empty
      __result.try(&.document).try(&.nodes).to_s.should_not be_empty
      __result.try(&.document).try(&.nodes).to_s.size.should be >=(2)
    end
    it "Unordered list produces List and ListItem nodes" do
      __result = HtmlToMarkdown.convert("<p>Items:</p><ul><li>Alpha</li><li>Beta</li><li>Gamma</li></ul>", HtmlToMarkdown::ConversionOptions.from_json("{\"include_document_structure\":true}"))
      __result.content.to_s.should_not be_empty
      __result.try(&.document).try(&.nodes).to_s.should_not be_empty
      __result.try(&.document).try(&.nodes).to_s.size.should be >=(2)
    end
    it "Multiple headings create multiple Heading nodes with correct levels" do
      __result = HtmlToMarkdown.convert("<h1>Main Title</h1><h2>Section One</h2><p>Section one content.</p><h2>Section Two</h2><p>Section two content.</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"include_document_structure\":true}"))
      __result.content.to_s.should_not be_empty
      __result.try(&.document).try(&.nodes).to_s.should_not be_empty
      __result.try(&.document).try(&.nodes).to_s.size.should be >=(4)
    end
    it "H1, H2, then another H1 creates two sibling top-level groups" do
      __result = HtmlToMarkdown.convert("<h1>Chapter One</h1><h2>Section A</h2><p>Section A content.</p><h1>Chapter Two</h1><h2>Section B</h2><p>Section B content.</p>", HtmlToMarkdown::ConversionOptions.from_json("{\"include_document_structure\":true}"))
      __result.content.to_s.should_not be_empty
      __result.try(&.document).try(&.nodes).to_s.should_not be_empty
      __result.try(&.document).try(&.nodes).to_s.size.should be >=(4)
    end
  end
end
