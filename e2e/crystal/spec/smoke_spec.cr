require "./spec_helper"

describe HtmlToMarkdown do
  describe "smoke" do
    it "Empty string produces empty output" do
      __result = HtmlToMarkdown.convert("", nil)
      __result.content.to_s.strip.should eq("")
    end
    it "H1 heading converts to ATX markdown" do
      __result = HtmlToMarkdown.convert("<h1>Title</h1>", nil)
      __result.content.to_s.should contain("# Title")
    end
    it "Simple paragraph converts correctly" do
      __result = HtmlToMarkdown.convert("<p>Hello World</p>", nil)
      __result.content.to_s.strip.should eq("Hello World")
      __result.content.to_s.should_not be_empty
    end
  end
end
